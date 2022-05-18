import * as z from 'zod'
import { NamespaceName } from '../../namespaces/util'
export const appEnvironmentsSchema = z.union([
  z.literal('local'),
  z.literal('development'),
  z.literal('staging'),
  z.literal('production')
])

export type Environment = z.infer<typeof appEnvironmentsSchema>
// This might change but make it the environment for now.
export type NamespaceOfApps = NamespaceName

export type Memory = `${number}${'E' | 'P' | 'T' | 'G' | 'M' | 'k' | 'm' | 'Ei' | 'Pi' | 'Ti' | 'Gi' | 'Mi' | 'Ki'}`

export type CPU = `${number}${'m'}`

export type ResourceType = 'infrastructure' | 'services'

export type ServiceName = 'graphql-mongo' | 'graphql-postgres' | 'grpc-mongo' | 'react-web'

export type ArgocdAppResourceName = `argocd-applications-children-${ResourceType}` | 'argocd-applications-parents'

const InfrastructureNames = [
  'namespace-names',
  'sealed-secrets',
  'cert-manager',
  'nginx-ingress',
  'linkerd',
  'linkerd-viz',
  'argocd'
] as const

export type InfrastructureName = typeof InfrastructureNames[number]

// A resource can have multiple kubernetes objects/resources e.g linkerd
// e.g linkerd can have different
export type ResourceName = InfrastructureName | ServiceName | ArgocdAppResourceName

export interface Settings<TAppName extends ServiceName> {
    requestMemory: Memory
    requestCpu: CPU
    limitMemory: Memory
    limitCpu: CPU
    replicaCount: number
    host: string
    image: `ghcr.io/oyelowo/${TAppName}:${string}`
}

export type RecursivePartial<T> = {
    [P in keyof T]?: T[P] extends (infer U)[]
        ? RecursivePartial<U>[]
        : T[P] extends object | undefined
        ? RecursivePartial<T[P]>
        : T[P]
}

// make all properties optional recursively including nested objects.
// keep in mind that this should be used on json / plain objects only.
// otherwise, it will make class methods optional as well.
export type DeepPartial<T> = {
    [P in keyof T]?: T[P] extends Array<infer I> ? Array<DeepPartial<I>> : DeepPartial<T[P]>
}

type MongoDb = 'mongodb'
type PostgresDb = 'postgresdb'
type DoesNotHaveDb = 'doesNotHaveDb'

export type DBType = MongoDb | PostgresDb | DoesNotHaveDb

export type MongoDbEnvVars<DBN extends `${ServiceName}-database`, NS extends NamespaceOfApps> = {
    dbType: MongoDb
    MONGODB_NAME: DBN
    MONGODB_USERNAME: string
    MONGODB_PASSWORD: string
    MONGODB_HOST: `${DBN}.${NS}`
    MONGODB_PORT: '27017'
    MONGODB_SERVICE_NAME: DBN
    MONGODB_STORAGE_CLASS: 'linode-block-storage-retain' // TODO: Do this properly
    MONGODB_ROOT_USERNAME: string
    MONGODB_ROOT_PASSWORD: string
}

export type PostgresDbEnvVars<DBN extends `${ServiceName}-database`, NS extends NamespaceOfApps> = {
    dbType: PostgresDb
    POSTGRES_NAME: DBN
    POSTGRES_DATABASE_NAME: DBN
    POSTGRES_USERNAME: string
    POSTGRES_PASSWORD: string
    POSTGRES_HOST: `${DBN}.${NS}`
    POSTGRES_PORT: '5432'
    POSTGRES_SERVICE_NAME: DBN
    POSTGRES_STORAGE_CLASS: string
}

type RedisServiceName<AN extends ServiceName> = `${AN}-redis`
type RedisServiceNameWithSuffix<AN extends ServiceName> = `${RedisServiceName<AN>}-master`

export type RedisDbEnvVars<AN extends ServiceName, NS extends NamespaceOfApps> = {
    REDIS_USERNAME?: string
    REDIS_PASSWORD?: string
    REDIS_HOST?: `${RedisServiceNameWithSuffix<AN>}.${NS}` // The application will also need this
    REDIS_SERVICE_NAME?: RedisServiceName<AN> // THIS is used redis helm chart config itself which adds a suffix(e.g master)
    REDIS_SERVICE_NAME_WITH_SUFFIX?: RedisServiceNameWithSuffix<AN>
    REDIS_PORT?: '6379'
}

type DatabaseEnvVars<DBN extends `${ServiceName}-database`, NS extends NamespaceOfApps> =
    | MongoDbEnvVars<DBN, NS>
    | PostgresDbEnvVars<DBN, NS>
    | { dbType: DoesNotHaveDb }

export type AppEnvVars<AN extends ServiceName, NS extends NamespaceOfApps> = {
    APP_ENVIRONMENT: Environment
    APP_HOST: '0.0.0.0'
    APP_PORT: '8000' | '50051' | '3000'
    // REDIS_USERNAME?: string;
    // REDIS_PASSWORD?: string;
    // // REDIS_HOST?: `${AN}-redis.${NS}`;
    // REDIS_HOST?: `${AN}-redis-master.${NS}`;
    // REDIS_SERVICE_NAME?: `${AN}-redis`; // TODO: Use a derivative approach for getting the host to prevent them from going out of sync
    // REDIS_SERVICE_NAME_WITH_SUFFIX?: `${AN}-redis-master`; // TODO: Use a derivative approach for getting the host to prevent them from going out of sync
    // REDIS_PORT?: "6379";
    GITHUB_CLIENT_ID?: string
    GITHUB_CLIENT_SECRET?: string
    GOOGLE_CLIENT_ID?: string
    GOOGLE_CLIENT_SECRET?: string
    NEXTAUTH_URL?: string
    GRAPHQL_MONGO_URL?: string // TODO: This could be make stronger typed
} & DatabaseEnvVars<`${AN}-database`, NS> &
    RedisDbEnvVars<AN, NS>

type EnvironmentVariables<AN extends ServiceName, NS extends NamespaceOfApps, DBT extends DBType> = Extract<
    AppEnvVars<AN, NS>,
    { dbType: DBT }
>

export type AppConfigs<AN extends ServiceName, DBT extends DBType, NS extends NamespaceOfApps> = {
    kubeConfig: Settings<NoUnion<AN>>
    envVars: Omit<EnvironmentVariables<AN, NS, DBT>, 'dbType'>
    metadata: {
        name: AN
        namespace: NS
    }
}

export type NoUnion<T, U = T> = T extends U ? ([U] extends [T] ? T : never) : never
