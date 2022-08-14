import { namespaces } from '../namespaces/index.js';
import { createArgocdApplication } from '../../shared/createArgoApplication.js';
import { getEnvironmentVariables } from '../../shared/validations.js';

const { ENVIRONMENT } = getEnvironmentVariables();
// App that deploys sealedSecretsController resources themselves
/* sealedSecretsController APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const sealedSecretsControllerApplication = createArgocdApplication({
    sourceAppDirectory: 'infrastructure/sealed-secrets',
    outputDirectory: 'infrastructure/argocd-applications-children-infrastructure',
    environment: ENVIRONMENT,
    namespace: namespaces.default,
});
