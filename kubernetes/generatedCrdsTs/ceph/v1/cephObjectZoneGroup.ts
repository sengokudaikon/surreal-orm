// *** WARNING: this file was generated by crd2pulumi. ***
// *** Do not edit by hand unless you're certain you know what you are doing! ***

import * as pulumi from "@pulumi/pulumi";
import { input as inputs, output as outputs } from "../../types";
import * as utilities from "../../utilities";

import {ObjectMeta} from "../../meta/v1";

/**
 * CephObjectZoneGroup represents a Ceph Object Store Gateway Zone Group
 */
export class CephObjectZoneGroup extends pulumi.CustomResource {
    /**
     * Get an existing CephObjectZoneGroup resource's state with the given name, ID, and optional extra
     * properties used to qualify the lookup.
     *
     * @param name The _unique_ name of the resulting resource.
     * @param id The _unique_ provider ID of the resource to lookup.
     * @param opts Optional settings to control the behavior of the CustomResource.
     */
    public static get(name: string, id: pulumi.Input<pulumi.ID>, opts?: pulumi.CustomResourceOptions): CephObjectZoneGroup {
        return new CephObjectZoneGroup(name, undefined as any, { ...opts, id: id });
    }

    /** @internal */
    public static readonly __pulumiType = 'kubernetes:ceph.rook.io/v1:CephObjectZoneGroup';

    /**
     * Returns true if the given object is an instance of CephObjectZoneGroup.  This is designed to work even
     * when multiple copies of the Pulumi SDK have been loaded into the same process.
     */
    public static isInstance(obj: any): obj is CephObjectZoneGroup {
        if (obj === undefined || obj === null) {
            return false;
        }
        return obj['__pulumiType'] === CephObjectZoneGroup.__pulumiType;
    }

    public readonly apiVersion!: pulumi.Output<"ceph.rook.io/v1" | undefined>;
    public readonly kind!: pulumi.Output<"CephObjectZoneGroup" | undefined>;
    public readonly metadata!: pulumi.Output<ObjectMeta>;
    /**
     * ObjectZoneGroupSpec represent the spec of an ObjectZoneGroup
     */
    public readonly spec!: pulumi.Output<outputs.ceph.v1.CephObjectZoneGroupSpec>;
    /**
     * Status represents the status of an object
     */
    public readonly status!: pulumi.Output<{[key: string]: any} | undefined>;

    /**
     * Create a CephObjectZoneGroup resource with the given unique name, arguments, and options.
     *
     * @param name The _unique_ name of the resource.
     * @param args The arguments to use to populate this resource's properties.
     * @param opts A bag of options that control this resource's behavior.
     */
    constructor(name: string, args?: CephObjectZoneGroupArgs, opts?: pulumi.CustomResourceOptions) {
        let resourceInputs: pulumi.Inputs = {};
        opts = opts || {};
        if (!opts.id) {
            resourceInputs["apiVersion"] = "ceph.rook.io/v1";
            resourceInputs["kind"] = "CephObjectZoneGroup";
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
        super(CephObjectZoneGroup.__pulumiType, name, resourceInputs, opts);
    }
}

/**
 * The set of arguments for constructing a CephObjectZoneGroup resource.
 */
export interface CephObjectZoneGroupArgs {
    apiVersion?: pulumi.Input<"ceph.rook.io/v1">;
    kind?: pulumi.Input<"CephObjectZoneGroup">;
    metadata?: pulumi.Input<ObjectMeta>;
    /**
     * ObjectZoneGroupSpec represent the spec of an ObjectZoneGroup
     */
    spec?: pulumi.Input<inputs.ceph.v1.CephObjectZoneGroupSpecArgs>;
    /**
     * Status represents the status of an object
     */
    status?: pulumi.Input<{[key: string]: any}>;
}
