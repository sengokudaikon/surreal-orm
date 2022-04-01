import { namespaceNames } from "./../shared/namespaces";
import { environmentVariables } from "./../shared/validations";
import { AppConfigs } from "../shared/types/own-types";
import { getSecretForApp } from "../../secretsManagement";

const environment = environmentVariables.ENVIRONMENT;
const secretsFromLocalConfigs = getSecretForApp("graphql-mongo");

// TODO: ADD A NEW KEY - SECRETS TO THE config which would accept secrets from the global secrets config used to generate manifests
export const graphqlMongoSettings: AppConfigs<"graphql-mongo", "mongodb", "applications"> = {
  kubeConfig: {
    requestMemory: "70Mi",
    requestCpu: "100m",
    limitMemory: "200Mi",
    limitCpu: "100m",
    replicaCount: 3,
    host: "0.0.0.0",
    image: `ghcr.io/oyelowo/graphql-mongo:${environmentVariables.IMAGE_TAG_GRAPHQL_MONGO}`,
  },

  envVars: {
    APP_ENVIRONMENT: environmentVariables.ENVIRONMENT,
    APP_HOST: "0.0.0.0",
    APP_PORT: "8000",

    MONGODB_NAME: "graphql-mongo-database",
    // TODO: remove these two. now coming handled in the deployment abstraction and uses referenced secret
    MONGODB_USERNAME: secretsFromLocalConfigs.MONGODB_USERNAME,
    MONGODB_PASSWORD: secretsFromLocalConfigs.MONGODB_PASSWORD,
    MONGODB_ROOT_USERNAME: secretsFromLocalConfigs.MONGODB_ROOT_USERNAME,
    MONGODB_ROOT_PASSWORD: secretsFromLocalConfigs.MONGODB_ROOT_PASSWORD,
    MONGODB_HOST: "graphql-mongo-database.applications",
    MONGODB_SERVICE_NAME: "graphql-mongo-database",
    MONGODB_STORAGE_CLASS: "linode-block-storage-retain",
    // hostAndPort":"graphql-mongo-0.mongo-graphql.development.svc.cluster.local:27017
    // MONGODB_HOST: "graphql-mongod-0.graphql-mongod-headless.development.svc.cluster.local",
    // const url = 'mongodb://username1:$[password]@mongo-graphql.development:27017/db1?authSource=$[authSource]';
    MONGODB_PORT: "27017",
  },
  metadata: {
    name: "graphql-mongo",
    namespace: namespaceNames.applications,
  },
};
