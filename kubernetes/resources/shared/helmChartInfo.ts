type Repo = "bitnami" | "jetspack" | "linkerd" | "sealedSecrets" | "argo"

type ChartInfo = {
    chart: string,
    version: string,
}

type ChartsInfo = Record<Repo, {
    repo: string,
    charts: Record<string, ChartInfo>
}>

// This function does nothing. It just helps with typing
export const checkConstType = <T extends ChartsInfo>(o: T) => o

export let helmChartsInfo = checkConstType({
    bitnami: {
        repo: 'https://charts.bitnami.com/bitnami',
        charts: {
            redis: {
                chart: 'redis',
                version: '16.8.9',
            },
            mongodb: {
                chart: 'mongodb',
                version: '11.1.10',
            },
            certManager: {
                chart: 'cert-manager',
                version: '0.5.0',
            },
            nginxIngress: {
                chart: 'nginx-ingress-controller',
                version: '9.1.26',
            },
            argocd: {
                chart: 'argo-cd',
                version: '3.1.16',
            },
            postgresql: {
                chart: 'postgresql',
                version: '11.6.7',
            },
            postgresqlHA: {
                chart: 'postgresql-ha',
                version: '9.1.6',
            }
        },
    },
    sealedSecrets: {
        repo: 'https://bitnami-labs.github.io/sealed-secrets',
        charts: {
            sealedSecrets: {
                chart: 'sealed-secrets',
                version: '2.1.7',
            }
        },
    },
    jetspack: {
        repo: 'https://charts.jetstack.io',
        charts: {
            certManager: {
                chart: 'cert-manager',
                version: 'v1.8.0',
            },
            certManagerTrust: {
                chart: 'cert-manager-trust',
                version: 'v0.1.1',
            }
        },
    },
    linkerd: {
        repo: 'https://helm.linkerd.io/stable',
        charts: {
            linkerd2: {
                chart: 'linkerd2',
                version: '2.11.2',
            },
            linkerdViz: {
                chart: 'linkerd-viz',
                version: '2.11.2',
            }
        },
    },
    argo: {
        repo: 'https://argoproj.github.io/argo-helm',
        charts: {
            argoCD: {
                chart: 'argo-cd',
                version: '4.5.3',
            }
        },
    },
} as const);
