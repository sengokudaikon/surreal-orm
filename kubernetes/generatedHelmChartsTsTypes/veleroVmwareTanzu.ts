// Don't Edit. This is autogenerated.
export interface IVeleroVmwareTanzu {
    image: Image;
    annotations: Annotations;
    labels: Annotations;
    podAnnotations: Annotations;
    podLabels: Annotations;
    resources: Resources;
    dnsPolicy: string;
    initContainers?: any;
    podSecurityContext: Annotations;
    containerSecurityContext: Annotations;
    lifecycle: Annotations;
    priorityClassName: string;
    terminationGracePeriodSeconds: number;
    tolerations: any[];
    affinity: Annotations;
    nodeSelector: Annotations;
    dnsConfig: Annotations;
    extraVolumes: any[];
    extraVolumeMounts: any[];
    extraObjects: any[];
    metrics: Metrics;
    kubectl: Kubectl;
    upgradeCRDs: boolean;
    cleanUpCRDs: boolean;
    configuration: Configuration;
    rbac: Rbac;
    serviceAccount: ServiceAccount;
    credentials: Credentials;
    backupsEnabled: boolean;
    snapshotsEnabled: boolean;
    deployRestic: boolean;
    restic: Restic;
    schedules: Annotations;
    configMaps: Annotations;
}
interface Restic {
    podVolumePath: string;
    privileged: boolean;
    priorityClassName: string;
    resources: Resources;
    tolerations: any[];
    annotations: Annotations;
    labels: Annotations;
    useScratchEmptyDir: boolean;
    extraVolumes: any[];
    extraVolumeMounts: any[];
    extraEnvVars: Annotations;
    dnsPolicy: string;
    podSecurityContext: PodSecurityContext;
    containerSecurityContext: Annotations;
    lifecycle: Annotations;
    nodeSelector: Annotations;
    affinity: Annotations;
    dnsConfig: Annotations;
}
interface PodSecurityContext {
    runAsUser: number;
}
interface Credentials {
    useSecret: boolean;
    name?: any;
    existingSecret?: any;
    secretContents: Annotations;
    extraEnvVars: Annotations;
    extraSecretRef: string;
}
interface ServiceAccount {
    server: Server;
}
interface Server {
    create: boolean;
    name?: any;
    annotations?: any;
    labels?: any;
}
interface Rbac {
    create: boolean;
    clusterAdministrator: boolean;
    clusterAdministratorName: string;
}
interface Configuration {
    provider?: any;
    backupStorageLocation: BackupStorageLocation;
    volumeSnapshotLocation: VolumeSnapshotLocation;
    backupSyncPeriod?: any;
    resticTimeout?: any;
    restoreResourcePriorities?: any;
    restoreOnlyMode?: any;
    clientQPS?: any;
    clientBurst?: any;
    clientPageSize?: any;
    disableControllers?: any;
    storeValidationFrequency?: any;
    garbageCollectionFrequency?: any;
    extraEnvVars: Annotations;
    features?: any;
    logLevel?: any;
    logFormat?: any;
    defaultVolumesToRestic?: any;
    defaultResticPruneFrequency?: any;
}
interface VolumeSnapshotLocation {
    name?: any;
    provider?: any;
    config: Annotations;
}
interface BackupStorageLocation {
    name?: any;
    provider?: any;
    bucket?: any;
    caCert?: any;
    prefix?: any;
    default?: any;
    accessMode: string;
    config: Annotations;
}
interface Kubectl {
    image: Image2;
    containerSecurityContext: Annotations;
    resources: Annotations;
    annotations: Annotations;
    labels: Annotations;
}
interface Image2 {
    repository: string;
}
interface Metrics {
    enabled: boolean;
    scrapeInterval: string;
    scrapeTimeout: string;
    service: Service;
    podAnnotations: PodAnnotations;
    serviceMonitor: ServiceMonitor;
    prometheusRule: PrometheusRule;
}
interface PrometheusRule {
    enabled: boolean;
    additionalLabels: Annotations;
    spec: any[];
}
interface ServiceMonitor {
    enabled: boolean;
    additionalLabels: Annotations;
}
interface PodAnnotations {
    'prometheus.io/scrape': string;
    'prometheus.io/port': string;
    'prometheus.io/path': string;
}
interface Service {
    annotations: Annotations;
    labels: Annotations;
}
interface Resources {
    requests: Requests;
    limits: Requests;
}
interface Requests {
    cpu: string;
    memory: string;
}
interface Annotations {}
interface Image {
    repository: string;
    tag: string;
    pullPolicy: string;
    imagePullSecrets: any[];
}
