// *** WARNING: this file was generated by crd2pulumi. ***
// *** Do not edit by hand unless you're certain you know what you are doing! ***

import * as pulumi from "@pulumi/pulumi";
import { input as inputs, output as outputs } from "../../types";
import * as utilities from "../../utilities";

import {ObjectMeta} from "../../meta/v1";

/**
 * CephObjectRealm represents a Ceph Object Store Gateway Realm
 */
export class CephObjectRealm extends pulumi.CustomResource {
    /**
     * Get an existing CephObjectRealm resource's state with the given name, ID, and optional extra
     * properties used to qualify the lookup.
     *
     * @param name The _unique_ name of the resulting resource.
     * @param id The _unique_ provider ID of the resource to lookup.
     * @param opts Optional settings to control the behavior of the CustomResource.
     */
    public static get(name: string, id: pulumi.Input<pulumi.ID>, opts?: pulumi.CustomResourceOptions): CephObjectRealm {
        return new CephObjectRealm(name, undefined as any, { ...opts, id: id });
    }

    /** @internal */
    public static readonly __pulumiType = 'kubernetes:ceph.rook.io/v1:CephObjectRealm';

    /**
     * Returns true if the given object is an instance of CephObjectRealm.  This is designed to work even
     * when multiple copies of the Pulumi SDK have been loaded into the same process.
     */
    public static isInstance(obj: any): obj is CephObjectRealm {
        if (obj === undefined || obj === null) {
            return false;
        }
        return obj['__pulumiType'] === CephObjectRealm.__pulumiType;
    }

    public readonly apiVersion!: pulumi.Output<"ceph.rook.io/v1" | undefined>;
    public readonly kind!: pulumi.Output<"CephObjectRealm" | undefined>;
    public readonly metadata!: pulumi.Output<ObjectMeta>;
    /**
     * ObjectRealmSpec represent the spec of an ObjectRealm
     */
    public readonly spec!: pulumi.Output<outputs.ceph.v1.CephObjectRealmSpec | undefined>;
    /**
     * Status represents the status of an object
     */
    public readonly status!: pulumi.Output<{[key: string]: any} | undefined>;

    /**
     * Create a CephObjectRealm resource with the given unique name, arguments, and options.
     *
     * @param name The _unique_ name of the resource.
     * @param args The arguments to use to populate this resource's properties.
     * @param opts A bag of options that control this resource's behavior.
     */
    constructor(name: string, args?: CephObjectRealmArgs, opts?: pulumi.CustomResourceOptions) {
        let resourceInputs: pulumi.Inputs = {};
        opts = opts || {};
        if (!opts.id) {
            resourceInputs["apiVersion"] = "ceph.rook.io/v1";
            resourceInputs["kind"] = "CephObjectRealm";
            resourceInputs["metadata"] = args ? args.metadata : undefined;
            resourceInputs["spec"] = args ? args.spec : undefined;
            resourceInputs["status"] = args ? args.status : undefined;
        } else {
            resourceInputs["apiVersion"] = undefined /*out*/;
            resourceInputs["kind"] = undefined /*out*/;
            resourceInputs["metadata"] = undefined /*out*/;
            resourceInputs["spec"] = undefined /*out*/;
            resourceInputs["status"] = undefined /*out*/;
        }
        opts = pulumi.mergeOptions(utilities.resourceOptsDefaults(), opts);
        super(CephObjectRealm.__pulumiType, name, resourceInputs, opts);
    }
}

/**
 * The set of arguments for constructing a CephObjectRealm resource.
 */
export interface CephObjectRealmArgs {
    apiVersion?: pulumi.Input<"ceph.rook.io/v1">;
    kind?: pulumi.Input<"CephObjectRealm">;
    metadata?: pulumi.Input<ObjectMeta>;
    /**
     * ObjectRealmSpec represent the spec of an ObjectRealm
     */
    spec?: pulumi.Input<inputs.ceph.v1.CephObjectRealmSpecArgs>;
    /**
     * Status represents the status of an object
     */
    status?: pulumi.Input<{[key: string]: any}>;
}
