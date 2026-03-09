import { useEffect, useMemo, useState, type ChangeEvent } from 'react'
import './App.css'
import DashboardSidebar from './components/DashboardSidebar'
import DashboardTopBar from './components/DashboardTopBar'
import DocumentWorkflowForm from './components/DocumentWorkflowForm'
import EngineResultsPanel from './components/EngineResultsPanel'
import EstateDocumentsForm from './components/EstateDocumentsForm'
import HealthChecksForm from './components/HealthChecksForm'
import OverviewCards from './components/OverviewCards'
import PayloadPanel from './components/PayloadPanel'
import RulesExplorerForm from './components/RulesExplorerForm'
import ScenarioBuilderForm from './components/ScenarioBuilderForm'
import ScenarioCandidatesForm from './components/ScenarioCandidatesForm'
import StatusMessage from './components/StatusMessage'
import {
  fetchHealthPayload,
  fetchRulesPayload,
  submitDocumentWorkflow,
  submitEstateDocumentAnalysis,
  submitScenarioCalculation,
  submitScenarioOptimization,
} from './lib/apiClient'
import {
  createScenarioAssetDraft,
  createScenarioDraft,
  scenarioDraftToApiPayload,
} from './lib/scenarioDraft'
import {
  inferDocumentFormat,
  readUploadPayload,
} from './lib/document'
import { logIntakeError, logIntakeEvent } from './lib/intakeLogger'
import { toJurisdictionPathToken } from './lib/jurisdiction'
import { getWorkflowFromPath, getWorkflowPath } from './lib/workflowRoutes'
import type {
  ApiEstateDocumentAnalysisResponse,
  ApiJurisdiction,
  ApiJurisdictionTaxRuleRegistryResponse,
  ApiOptimizedScenario,
  ApiScenarioResult,
  ApiTaxRuleRegistryEntry,
  ApiVersionedJurisdictionTaxRuleSet,
  RunMode,
  StatusTone,
  Workflow,
} from './types/api'
import type {
  ScenarioBuilderAssetDraft,
  ScenarioBuilderDraft,
} from './types/scenarioBuilder'

const DEFAULT_STATUS_TEXT = 'Choose a workflow and run an operation.'
const DEFAULT_SUMMARY_TEXT = 'No engine capability has been executed yet.'
const DEFAULT_PAYLOAD_PREVIEW = 'Response payload will appear here.'

type WorkspaceInsightFact = { label: string; value: string }

type WorkflowWorkspace = {
  statusTone: StatusTone
  statusText: string
  summaryText: string
  payloadPreview: string
  insightFacts: WorkspaceInsightFact[]
  insightBadges: string[]
  documentResults: ApiScenarioResult[]
  singleScenarioResult: ApiScenarioResult | null
  optimizedScenario: ApiOptimizedScenario | null
  estateAnalysis: ApiEstateDocumentAnalysisResponse | null
}

function createEmptyWorkspace(): WorkflowWorkspace {
  return {
    statusTone: 'idle',
    statusText: DEFAULT_STATUS_TEXT,
    summaryText: DEFAULT_SUMMARY_TEXT,
    payloadPreview: DEFAULT_PAYLOAD_PREVIEW,
    insightFacts: [],
    insightBadges: [],
    documentResults: [],
    singleScenarioResult: null,
    optimizedScenario: null,
    estateAnalysis: null,
  }
}

function createInitialWorkspaces(): Record<Workflow, WorkflowWorkspace> {
  return {
    document: createEmptyWorkspace(),
    scenario: createEmptyWorkspace(),
    optimize: createEmptyWorkspace(),
    'estate-docs': createEmptyWorkspace(),
    rules: createEmptyWorkspace(),
    health: createEmptyWorkspace(),
  }
}

function clearWorkspaceOutputs(workspace: WorkflowWorkspace): WorkflowWorkspace {
  return {
    ...workspace,
    insightFacts: [],
    insightBadges: [],
    documentResults: [],
    singleScenarioResult: null,
    optimizedScenario: null,
    estateAnalysis: null,
  }
}

function App() {
  const [workflow, setWorkflow] = useState<Workflow>(() => getWorkflowFromPath(window.location.pathname))
  const [mode, setMode] = useState<RunMode>('ingest')
  const [file, setFile] = useState<File | null>(null)
  const [estateFiles, setEstateFiles] = useState<File[]>([])
  const [scenarioDraft, setScenarioDraft] = useState<ScenarioBuilderDraft>(() =>
    createScenarioDraft('Scenario Calculation'),
  )
  const [optimizationDrafts, setOptimizationDrafts] = useState<ScenarioBuilderDraft[]>(() => [
    createScenarioDraft('Current Plan'),
    createScenarioDraft('Alternative Plan'),
  ])
  const [jurisdictions, setJurisdictions] = useState<ApiJurisdiction[]>([])
  const [selectedJurisdiction, setSelectedJurisdiction] = useState<ApiJurisdiction>('SouthAfrica')
  const [taxYear, setTaxYear] = useState('2025')

  const [workspaces, setWorkspaces] = useState<Record<Workflow, WorkflowWorkspace>>(() =>
    createInitialWorkspaces(),
  )
  const [documentInputVersion, setDocumentInputVersion] = useState(0)
  const [estateInputVersion, setEstateInputVersion] = useState(0)
  const [isSubmitting, setIsSubmitting] = useState(false)
  const activeWorkspace = workspaces[workflow]

  useEffect(() => {
    const syncWorkflowFromHistory = () => {
      setWorkflow(getWorkflowFromPath(window.location.pathname))
    }

    window.addEventListener('popstate', syncWorkflowFromHistory)
    return () => window.removeEventListener('popstate', syncWorkflowFromHistory)
  }, [])

  useEffect(() => {
    const expectedPath = getWorkflowPath(workflow)

    if (window.location.pathname !== expectedPath) {
      window.history.replaceState({ workflow }, '', expectedPath)
    }
  }, [workflow])

  const toneClass = useMemo(() => {
    if (activeWorkspace.statusTone === 'error') {
      return 'status status-error'
    }
    if (activeWorkspace.statusTone === 'success') {
      return 'status status-success'
    }
    if (activeWorkspace.statusTone === 'busy') {
      return 'status status-busy'
    }
    return 'status'
  }, [activeWorkspace.statusTone])

  const updateWorkspace = (
    targetWorkflow: Workflow,
    updater: (workspace: WorkflowWorkspace) => WorkflowWorkspace,
  ) => {
    setWorkspaces((current) => ({
      ...current,
      [targetWorkflow]: updater(current[targetWorkflow]),
    }))
  }

  const resetWorkspace = (targetWorkflow: Workflow) => {
    updateWorkspace(targetWorkflow, () => createEmptyWorkspace())

    if (targetWorkflow === 'document') {
      setMode('ingest')
      setFile(null)
      setDocumentInputVersion((current) => current + 1)
      return
    }

    if (targetWorkflow === 'scenario') {
      setScenarioDraft(createScenarioDraft('Scenario Calculation'))
      return
    }

    if (targetWorkflow === 'optimize') {
      setOptimizationDrafts([
        createScenarioDraft('Current Plan'),
        createScenarioDraft('Alternative Plan'),
      ])
      return
    }

    if (targetWorkflow === 'estate-docs') {
      setEstateFiles([])
      setEstateInputVersion((current) => current + 1)
      return
    }

    if (targetWorkflow === 'rules') {
      setJurisdictions([])
      setSelectedJurisdiction('SouthAfrica')
      setTaxYear('2025')
    }
  }

  const setWorkspaceFailure = (targetWorkflow: Workflow, message: string, rawText: string) => {
    updateWorkspace(targetWorkflow, (current) => ({
      ...clearWorkspaceOutputs(current),
      statusTone: 'error',
      statusText: message,
      summaryText: 'Operation failed.',
      payloadPreview: rawText.length > 0 ? rawText : 'Empty response body from server.',
    }))
  }

  const stagedInputCount =
    workflow === 'document'
      ? Number(file !== null)
      : workflow === 'scenario'
        ? scenarioDraft.assets.length
        : workflow === 'optimize'
          ? optimizationDrafts.length
          : workflow === 'estate-docs'
            ? estateFiles.length
            : workflow === 'rules'
              ? jurisdictions.length
              : 2

  const stagedInputHelper =
    workflow === 'document'
      ? 'Scenario documents staged in this workspace'
      : workflow === 'scenario'
        ? 'Assets in the current scenario draft'
        : workflow === 'optimize'
          ? 'Candidate scenarios in the optimizer workspace'
          : workflow === 'estate-docs'
            ? 'Estate documents staged in this workspace'
            : workflow === 'rules'
              ? 'Jurisdictions loaded into the rules workspace'
              : 'Available service checks in this workspace'

  const workspaceOutputValue =
    workflow === 'estate-docs' && activeWorkspace.estateAnalysis
      ? `${Math.round(activeWorkspace.estateAnalysis.readiness_score * 100)}%`
      : String(
          activeWorkspace.documentResults.length +
            (activeWorkspace.singleScenarioResult ? 1 : 0) +
            (activeWorkspace.optimizedScenario ? 1 : 0),
        )

  const workspaceOutputHelper =
    workflow === 'estate-docs'
      ? activeWorkspace.estateAnalysis
        ? 'Latest readiness score for this workspace'
        : 'No estate intake result yet'
      : 'Outputs stored in the current workspace'

  const overviewCards = useMemo(
    () => {
      const cards = [
        {
          label: 'Workspace Status',
          value: activeWorkspace.statusTone[0].toUpperCase() + activeWorkspace.statusTone.slice(1),
          helper: activeWorkspace.statusText,
        },
        {
          label: 'Staged Inputs',
          value: String(stagedInputCount),
          helper: stagedInputHelper,
        },
      ]

      if (workflow === 'document') {
        cards.push({
          label: 'Run Profile',
          value: mode === 'calculate' ? 'Full Run' : 'Validate',
          helper:
            mode === 'calculate'
              ? 'Validation followed by calculation output'
              : 'Validation-only intake path',
        })
      }

      cards.push({
        label: workflow === 'estate-docs' ? 'Estate Readiness' : 'Workspace Outputs',
        value: workspaceOutputValue,
        helper: workspaceOutputHelper,
      })

      return cards
    },
    [
      activeWorkspace.statusText,
      activeWorkspace.statusTone,
      mode,
      stagedInputCount,
      stagedInputHelper,
      workflow,
      workspaceOutputHelper,
      workspaceOutputValue,
    ],
  )

  const handleFileChange = (event: ChangeEvent<HTMLInputElement>) => {
    const nextFile = event.target.files?.[0] ?? null
    setFile(nextFile)

    if (!nextFile) {
      logIntakeEvent('document_file_cleared')
      updateWorkspace('document', (current) => ({
        ...current,
        statusTone: 'idle',
        statusText: DEFAULT_STATUS_TEXT,
      }))
      return
    }

    const format = inferDocumentFormat(nextFile)
    if (!format) {
      logIntakeError('document_file_invalid', {
        file_name: nextFile.name,
        file_size_bytes: nextFile.size,
        mime_type: nextFile.type,
      })
      updateWorkspace('document', (current) => ({
        ...current,
        statusTone: 'error',
        statusText: `Unsupported file type: ${nextFile.name}`,
      }))
      return
    }

    logIntakeEvent('document_file_selected', {
      file_name: nextFile.name,
      file_size_bytes: nextFile.size,
      mime_type: nextFile.type,
      format,
    })
    updateWorkspace('document', (current) => ({
      ...current,
      statusTone: 'idle',
      statusText: `Selected: ${nextFile.name} (${format})`,
    }))
  }

  const handleEstateFilesChange = (event: ChangeEvent<HTMLInputElement>) => {
    const nextFiles = Array.from(event.target.files ?? [])
    setEstateFiles(nextFiles)

    if (nextFiles.length === 0) {
      logIntakeEvent('estate_batch_cleared')
      updateWorkspace('estate-docs', (current) => ({
        ...current,
        statusTone: 'idle',
        statusText: DEFAULT_STATUS_TEXT,
      }))
      return
    }

    const unsupported = nextFiles.find((item) => inferDocumentFormat(item) === null)
    if (unsupported) {
      logIntakeError('estate_batch_invalid_file', {
        file_name: unsupported.name,
        file_size_bytes: unsupported.size,
        mime_type: unsupported.type,
      })
      updateWorkspace('estate-docs', (current) => ({
        ...current,
        statusTone: 'error',
        statusText: `Unsupported file type: ${unsupported.name}`,
      }))
      return
    }

    logIntakeEvent('estate_batch_selected', {
      files: nextFiles.map((item) => ({
        name: item.name,
        size_bytes: item.size,
        mime_type: item.type,
      })),
    })
    updateWorkspace('estate-docs', (current) => ({
      ...current,
      statusTone: 'idle',
      statusText: `Selected ${nextFiles.length} estate document(s).`,
    }))
  }

  const updateScenarioDraftField = <K extends keyof ScenarioBuilderDraft>(
    field: K,
    value: ScenarioBuilderDraft[K],
  ) => {
    setScenarioDraft((current) => ({ ...current, [field]: value }))
  }

  const updateScenarioAssetField = (
    assetId: string,
    field: keyof ScenarioBuilderAssetDraft,
    value: string | boolean,
  ) => {
    setScenarioDraft((current) => ({
      ...current,
      assets: current.assets.map((asset) =>
        asset.id === assetId ? { ...asset, [field]: value } : asset,
      ),
    }))
  }

  const addScenarioAsset = () => {
    setScenarioDraft((current) => ({
      ...current,
      assets: [...current.assets, createScenarioAssetDraft()],
    }))
  }

  const removeScenarioAsset = (assetId: string) => {
    setScenarioDraft((current) => ({
      ...current,
      assets: current.assets.filter((asset) => asset.id !== assetId),
    }))
  }

  const updateOptimizationDraftField = <K extends keyof ScenarioBuilderDraft>(
    index: number,
    field: K,
    value: ScenarioBuilderDraft[K],
  ) => {
    setOptimizationDrafts((current) =>
      current.map((draft, itemIndex) =>
        itemIndex === index ? { ...draft, [field]: value } : draft,
      ),
    )
  }

  const updateOptimizationAssetField = (
    index: number,
    assetId: string,
    field: keyof ScenarioBuilderAssetDraft,
    value: string | boolean,
  ) => {
    setOptimizationDrafts((current) =>
      current.map((draft, itemIndex) =>
        itemIndex === index
          ? {
              ...draft,
              assets: draft.assets.map((asset) =>
                asset.id === assetId ? { ...asset, [field]: value } : asset,
              ),
            }
          : draft,
      ),
    )
  }

  const addOptimizationAsset = (index: number) => {
    setOptimizationDrafts((current) =>
      current.map((draft, itemIndex) =>
        itemIndex === index
          ? { ...draft, assets: [...draft.assets, createScenarioAssetDraft()] }
          : draft,
      ),
    )
  }

  const removeOptimizationAsset = (index: number, assetId: string) => {
    setOptimizationDrafts((current) =>
      current.map((draft, itemIndex) =>
        itemIndex === index
          ? {
              ...draft,
              assets: draft.assets.filter((asset) => asset.id !== assetId),
            }
          : draft,
      ),
    )
  }

  const addOptimizationScenario = () => {
    setOptimizationDrafts((current) => [
      ...current,
      createScenarioDraft(`Candidate ${current.length + 1}`),
    ])
  }

  const removeOptimizationScenario = (index: number) => {
    setOptimizationDrafts((current) => current.filter((_, itemIndex) => itemIndex !== index))
  }

  const runDocumentWorkflow = async () => {
    const targetWorkflow: Workflow = 'document'

    if (!file) {
      updateWorkspace(targetWorkflow, (current) => ({
        ...current,
        statusTone: 'error',
        statusText: 'Select a file before processing.',
      }))
      return
    }

    const format = inferDocumentFormat(file)
    if (!format) {
      updateWorkspace(targetWorkflow, (current) => ({
        ...current,
        statusTone: 'error',
        statusText: 'Unsupported file type. Use PDF, DOCX, CSV, TXT, or JSON.',
      }))
      return
    }

    setIsSubmitting(true)
    updateWorkspace(targetWorkflow, (current) => ({
      ...clearWorkspaceOutputs(current),
      statusTone: 'busy',
      statusText: 'Reading document...',
      summaryText: 'Processing document workflow...',
      payloadPreview: '',
    }))

    try {
      const uploadPayload = await readUploadPayload(file, format)
      const outcome = await submitDocumentWorkflow({
        documentName: file.name,
        format,
        mode,
        uploadPayload,
      })

      if ('message' in outcome) {
        logIntakeError('document_workflow_ui_failed', {
          document_name: file.name,
          message: outcome.message,
        })
        setWorkspaceFailure(targetWorkflow, outcome.message, outcome.rawText)
        return
      }

      updateWorkspace(targetWorkflow, (current) => ({
        ...current,
        payloadPreview: JSON.stringify(outcome.payload, null, 2),
      }))

      if (outcome.kind === 'analysis') {
        const readinessPercent = Math.round((outcome.payload.readiness_score ?? 0) * 100)
        const missingRequiredCount = outcome.payload.missing_required_document_types.length
        logIntakeEvent('document_workflow_ui_completed', {
          document_name: file.name,
          kind: 'analysis',
          readiness_percent: readinessPercent,
          missing_required_types: missingRequiredCount,
        })
        updateWorkspace(targetWorkflow, (current) => ({
          ...current,
          estateAnalysis: outcome.payload,
          insightFacts: [
            { label: 'Checklist Readiness', value: `${readinessPercent}%` },
            { label: 'Missing Required Types', value: String(missingRequiredCount) },
            { label: 'Detected Documents', value: String(outcome.payload.detections.length) },
            { label: 'Checklist Items', value: String(outcome.payload.checklist.length) },
          ],
          insightBadges: outcome.payload.missing_required_document_types.slice(0, 6),
          statusTone: 'success',
          statusText: 'Document routed to estate legal/tax intake analysis.',
          summaryText: 'Estate intake checklist analyzed.',
        }))
        return
      }

      if (outcome.kind === 'calculate') {
        logIntakeEvent('document_workflow_ui_completed', {
          document_name: file.name,
          kind: 'calculate',
          scenarios: outcome.payload.scenarios.length,
          results: outcome.payload.results.length,
        })
        updateWorkspace(targetWorkflow, (current) => ({
          ...current,
          documentResults: outcome.payload.results ?? [],
          statusTone: 'success',
          statusText: 'Document parsed, validated, and calculated.',
          summaryText: `Parsed ${outcome.payload.scenarios.length} scenario(s), calculated ${outcome.payload.results.length} result(s).`,
        }))
        return
      }

      logIntakeEvent('document_workflow_ui_completed', {
        document_name: file.name,
        kind: 'ingest',
        scenarios: outcome.payload.scenarios.length,
      })
      updateWorkspace(targetWorkflow, (current) => ({
        ...current,
        statusTone: 'success',
        statusText: 'Document parsed successfully.',
        summaryText: `Parsed and validated ${outcome.payload.scenarios.length} scenario(s).`,
      }))
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Unknown error'
      logIntakeError('document_workflow_ui_exception', {
        document_name: file.name,
        message,
      })
      setWorkspaceFailure(targetWorkflow, `Unexpected error: ${message}`, '')
    } finally {
      setIsSubmitting(false)
    }
  }

  const runScenarioCalculation = async () => {
    const targetWorkflow: Workflow = 'scenario'

    setIsSubmitting(true)
    updateWorkspace(targetWorkflow, (current) => ({
      ...clearWorkspaceOutputs(current),
      statusTone: 'busy',
      statusText: 'Calculating scenario...',
      summaryText: 'Running direct scenario calculation...',
      payloadPreview: '',
    }))

    try {
      const payload = scenarioDraftToApiPayload(scenarioDraft)
      const outcome = await submitScenarioCalculation(payload)
      if (!outcome.ok) {
        setWorkspaceFailure(targetWorkflow, outcome.message, outcome.rawText)
        return
      }

      updateWorkspace(targetWorkflow, (current) => ({
        ...current,
        singleScenarioResult: outcome.payload,
        payloadPreview: JSON.stringify(outcome.payload, null, 2),
        statusTone: 'success',
        statusText: 'Scenario calculation completed.',
        summaryText: 'Direct scenario calculation result loaded.',
      }))
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Unknown error'
      setWorkspaceFailure(targetWorkflow, `Unexpected error: ${message}`, '')
    } finally {
      setIsSubmitting(false)
    }
  }

  const runScenarioOptimization = async () => {
    const targetWorkflow: Workflow = 'optimize'

    if (optimizationDrafts.length < 2) {
      updateWorkspace(targetWorkflow, (current) => ({
        ...current,
        statusTone: 'error',
        statusText: 'Add at least two candidate scenarios before optimization.',
      }))
      return
    }

    setIsSubmitting(true)
    updateWorkspace(targetWorkflow, (current) => ({
      ...clearWorkspaceOutputs(current),
      statusTone: 'busy',
      statusText: 'Optimizing candidate scenarios...',
      summaryText: 'Running scenario optimization...',
      payloadPreview: '',
    }))

    try {
      const payload = optimizationDrafts.map((draft) => scenarioDraftToApiPayload(draft))
      const outcome = await submitScenarioOptimization(payload)
      if (!outcome.ok) {
        setWorkspaceFailure(targetWorkflow, outcome.message, outcome.rawText)
        return
      }

      updateWorkspace(targetWorkflow, (current) => ({
        ...current,
        optimizedScenario: outcome.payload,
        payloadPreview: JSON.stringify(outcome.payload, null, 2),
        statusTone: 'success',
        statusText: 'Scenario optimization completed.',
        summaryText: outcome.payload
          ? `Selected scenario index ${outcome.payload.index} with ${outcome.payload.score.liquidity_risk_band} liquidity risk.`
          : 'No optimized scenario was returned.',
      }))
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Unknown error'
      setWorkspaceFailure(targetWorkflow, `Unexpected error: ${message}`, '')
    } finally {
      setIsSubmitting(false)
    }
  }

  const runEstateAnalysis = async () => {
    const targetWorkflow: Workflow = 'estate-docs'

    if (estateFiles.length === 0) {
      updateWorkspace(targetWorkflow, (current) => ({
        ...current,
        statusTone: 'error',
        statusText: 'Select one or more estate documents before analysis.',
      }))
      return
    }

    setIsSubmitting(true)
    updateWorkspace(targetWorkflow, (current) => ({
      ...clearWorkspaceOutputs(current),
      statusTone: 'busy',
      statusText: 'Preparing estate document batch...',
      summaryText: 'Running estate legal/tax intake analysis...',
      payloadPreview: '',
    }))

    try {
      const documents = await Promise.all(
        estateFiles.map(async (item) => {
          const format = inferDocumentFormat(item)
          if (!format) {
            throw new Error(`Unsupported file type: ${item.name}`)
          }

          const payload = await readUploadPayload(item, format)
          return {
            document_name: item.name,
            format,
            ...payload,
          }
        }),
      )

      const outcome = await submitEstateDocumentAnalysis(documents)
      if (!outcome.ok) {
        logIntakeError('estate_batch_ui_failed', {
          files: estateFiles.map((item) => item.name),
          message: outcome.message,
        })
        setWorkspaceFailure(targetWorkflow, outcome.message, outcome.rawText)
        return
      }

      logIntakeEvent('estate_batch_ui_completed', {
        files: estateFiles.map((item) => item.name),
        readiness_percent: Math.round(outcome.payload.readiness_score * 100),
        checklist_items: outcome.payload.checklist.length,
      })
      const readinessPercent = Math.round(outcome.payload.readiness_score * 100)
      const missingRequiredCount = outcome.payload.missing_required_document_types.length
      updateWorkspace(targetWorkflow, (current) => ({
        ...current,
        estateAnalysis: outcome.payload,
        insightFacts: [
          { label: 'Checklist Readiness', value: `${readinessPercent}%` },
          { label: 'Missing Required Types', value: String(missingRequiredCount) },
          { label: 'Detected Documents', value: String(outcome.payload.detections.length) },
          { label: 'Checklist Items', value: String(outcome.payload.checklist.length) },
        ],
        insightBadges: outcome.payload.missing_required_document_types.slice(0, 6),
        payloadPreview: JSON.stringify(outcome.payload, null, 2),
        statusTone: 'success',
        statusText: 'Estate legal/tax intake analysis completed.',
        summaryText: 'Estate legal/tax intake analysis completed.',
      }))
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Unknown error'
      logIntakeError('estate_batch_ui_exception', {
        files: estateFiles.map((item) => item.name),
        message,
      })
      setWorkspaceFailure(targetWorkflow, `Unexpected error: ${message}`, '')
    } finally {
      setIsSubmitting(false)
    }
  }

  const runRulesRequest = async (endpoint: string, successText: string, summaryFallback: string) => {
    const targetWorkflow: Workflow = 'rules'

    setIsSubmitting(true)
    updateWorkspace(targetWorkflow, (current) => ({
      ...clearWorkspaceOutputs(current),
      statusTone: 'busy',
      statusText: 'Loading rules data...',
      summaryText: 'Querying rules endpoints...',
      payloadPreview: '',
    }))

    try {
      const outcome = await fetchRulesPayload(endpoint)
      if (!outcome.ok) {
        setWorkspaceFailure(targetWorkflow, outcome.message, outcome.rawText)
        return
      }

      updateWorkspace(targetWorkflow, (current) => ({
        ...current,
        payloadPreview: JSON.stringify(outcome.payload, null, 2),
        statusTone: 'success',
        statusText: successText,
      }))

      if (Array.isArray(outcome.payload)) {
        if (outcome.payload.every((item) => typeof item === 'string')) {
          const items = outcome.payload as ApiJurisdiction[]
          setJurisdictions(items)
          setSelectedJurisdiction(items[0] ?? 'SouthAfrica')
          updateWorkspace(targetWorkflow, (current) => ({
            ...current,
            insightFacts: [{ label: 'Supported Jurisdictions', value: String(items.length) }],
            insightBadges: items,
            summaryText: `Loaded ${items.length} supported jurisdiction(s).`,
          }))
          return
        }

        const registryEntries = outcome.payload as ApiTaxRuleRegistryEntry[]
        updateWorkspace(targetWorkflow, (current) => ({
          ...current,
          insightFacts: [{ label: 'Registry Entries', value: String(registryEntries.length) }],
          summaryText: `Loaded ${registryEntries.length} registry entr${registryEntries.length === 1 ? 'y' : 'ies'}.`,
        }))
        return
      }

      if ('versions' in outcome.payload) {
        const registryPayload = outcome.payload as ApiJurisdictionTaxRuleRegistryResponse
        updateWorkspace(targetWorkflow, (current) => ({
          ...current,
          insightFacts: [
            { label: 'Jurisdiction', value: registryPayload.jurisdiction },
            {
              label: 'Supported Tax Years',
              value: `${registryPayload.supported_tax_year_from} - ${registryPayload.supported_tax_year_to}`,
            },
            { label: 'Registered Versions', value: String(registryPayload.versions.length) },
            { label: 'Latest Version', value: registryPayload.latest_version_id },
          ],
        }))
      } else if ('version' in outcome.payload) {
        const versionedPayload = outcome.payload as ApiVersionedJurisdictionTaxRuleSet
        updateWorkspace(targetWorkflow, (current) => ({
          ...current,
          insightFacts: [
            { label: 'Jurisdiction', value: versionedPayload.jurisdiction },
            { label: 'Tax Year', value: String(versionedPayload.supported_tax_year) },
            { label: 'Version', value: versionedPayload.version.version_id },
          ],
        }))
      }

      updateWorkspace(targetWorkflow, (current) => ({
        ...current,
        summaryText: summaryFallback,
      }))
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Unknown error'
      setWorkspaceFailure(targetWorkflow, `Unexpected error: ${message}`, '')
    } finally {
      setIsSubmitting(false)
    }
  }

  const runHealthCheck = async (endpoint: '/health' | '/health/db', successText: string) => {
    const targetWorkflow: Workflow = 'health'

    setIsSubmitting(true)
    updateWorkspace(targetWorkflow, (current) => ({
      ...clearWorkspaceOutputs(current),
      statusTone: 'busy',
      statusText: 'Running health check...',
      summaryText: 'Checking service readiness...',
      payloadPreview: '',
    }))

    try {
      const outcome = await fetchHealthPayload(endpoint)
      if (!outcome.ok) {
        setWorkspaceFailure(targetWorkflow, outcome.message, outcome.rawText)
        return
      }

      updateWorkspace(targetWorkflow, (current) => ({
        ...current,
        payloadPreview: JSON.stringify(outcome.payload, null, 2),
        statusTone: 'success',
        statusText: successText,
        insightFacts: [{ label: 'Status', value: outcome.payload.status }],
        summaryText: `Service status: ${outcome.payload.status}.`,
      }))
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Unknown error'
      setWorkspaceFailure(targetWorkflow, `Unexpected error: ${message}`, '')
    } finally {
      setIsSubmitting(false)
    }
  }

  const handleWorkflowChange = (nextWorkflow: Workflow) => {
    setWorkflow(nextWorkflow)

    const nextPath = getWorkflowPath(nextWorkflow)
    if (window.location.pathname !== nextPath) {
      window.history.pushState({ workflow: nextWorkflow }, '', nextPath)
    }
  }

  const renderControls = () => {
    if (workflow === 'document') {
      return (
        <DocumentWorkflowForm
          inputVersion={documentInputVersion}
          file={file}
          mode={mode}
          isSubmitting={isSubmitting}
          onModeChange={setMode}
          onFileChange={handleFileChange}
          onProcess={runDocumentWorkflow}
          onClear={() => resetWorkspace('document')}
        />
      )
    }

    if (workflow === 'scenario') {
      return (
        <ScenarioBuilderForm
          draft={scenarioDraft}
          isSubmitting={isSubmitting}
          onDraftChange={updateScenarioDraftField}
          onAssetChange={updateScenarioAssetField}
          onAddAsset={addScenarioAsset}
          onRemoveAsset={removeScenarioAsset}
          onSubmit={runScenarioCalculation}
        />
      )
    }

    if (workflow === 'optimize') {
      return (
        <ScenarioCandidatesForm
          drafts={optimizationDrafts}
          isSubmitting={isSubmitting}
          onDraftFieldChange={updateOptimizationDraftField}
          onAssetFieldChange={updateOptimizationAssetField}
          onAddAsset={addOptimizationAsset}
          onRemoveAsset={removeOptimizationAsset}
          onAddScenario={addOptimizationScenario}
          onRemoveScenario={removeOptimizationScenario}
          onSubmit={runScenarioOptimization}
        />
      )
    }

    if (workflow === 'estate-docs') {
      return (
        <EstateDocumentsForm
          inputVersion={estateInputVersion}
          files={estateFiles}
          isSubmitting={isSubmitting}
          onChange={handleEstateFilesChange}
          onSubmit={runEstateAnalysis}
        />
      )
    }

    if (workflow === 'rules') {
      const jurisdictionToken = toJurisdictionPathToken(selectedJurisdiction)

      return (
        <RulesExplorerForm
          jurisdictions={jurisdictions}
          selectedJurisdiction={selectedJurisdiction}
          taxYear={taxYear}
          isSubmitting={isSubmitting}
          onJurisdictionChange={setSelectedJurisdiction}
          onTaxYearChange={setTaxYear}
          onLoadJurisdictions={() =>
            runRulesRequest('/v1/jurisdictions', 'Jurisdictions loaded.', 'Loaded jurisdiction list.')
          }
          onLoadRegistry={() =>
            runRulesRequest('/v1/rules/registry', 'Rule registry loaded.', 'Loaded rule registry.')
          }
          onLoadJurisdictionRegistry={() =>
            runRulesRequest(
              `/v1/rules/registry/${jurisdictionToken}`,
              'Jurisdiction registry loaded.',
              `Loaded rule registry for ${selectedJurisdiction}.`,
            )
          }
          onLoadLatestRules={() =>
            runRulesRequest(
              `/v1/rules/latest/${jurisdictionToken}`,
              'Latest rules loaded.',
              `Loaded latest rules for ${selectedJurisdiction}.`,
            )
          }
          onLoadRulesByYear={() =>
            runRulesRequest(
              `/v1/rules/${jurisdictionToken}/${encodeURIComponent(taxYear)}`,
              'Tax-year rules loaded.',
              `Loaded rules for ${selectedJurisdiction} tax year ${taxYear}.`,
            )
          }
        />
      )
    }

    return (
      <HealthChecksForm
        isSubmitting={isSubmitting}
        onApiHealth={() => runHealthCheck('/health', 'API health check passed.')}
        onDatabaseHealth={() => runHealthCheck('/health/db', 'Database health check passed.')}
      />
    )
  }

  return (
    <main className="dashboard-shell">
      <DashboardSidebar workflow={workflow} onWorkflowChange={handleWorkflowChange} />

      <div className="dashboard-main">
        <DashboardTopBar
          workflow={workflow}
          statusTone={activeWorkspace.statusTone}
          statusText={activeWorkspace.statusText}
          onReset={() => resetWorkspace(workflow)}
          isSubmitting={isSubmitting}
        />

        <div className="dashboard-content">
          <section className="dashboard-column dashboard-column-primary">
            <section className="panel workspace-panel">
              {renderControls()}
              {workflow !== 'document' && (
                <div className="actions">
                  <button
                    className="btn btn-secondary"
                    type="button"
                    onClick={() => resetWorkspace(workflow)}
                    disabled={isSubmitting}
                  >
                    Reset Output
                  </button>
                </div>
              )}
              <StatusMessage className={toneClass} text={activeWorkspace.statusText} />
            </section>
          </section>

          <section className="dashboard-column dashboard-column-secondary">
            <OverviewCards cards={overviewCards} />
            <section className="panel results-panel-shell">
              <EngineResultsPanel
                summaryText={activeWorkspace.summaryText}
                insightFacts={activeWorkspace.insightFacts}
                insightBadges={activeWorkspace.insightBadges}
                documentResults={activeWorkspace.documentResults}
                singleScenarioResult={activeWorkspace.singleScenarioResult}
                optimizedScenario={activeWorkspace.optimizedScenario}
                estateAnalysis={activeWorkspace.estateAnalysis}
              />
            </section>
            <PayloadPanel payloadPreview={activeWorkspace.payloadPreview} />
          </section>
        </div>
      </div>
    </main>
  )
}

export default App
