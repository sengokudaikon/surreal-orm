// *** WARNING: this file was generated by crd2pulumi. ***
// *** Do not edit by hand unless you're certain you know what you are doing! ***

import * as pulumi from "@pulumi/pulumi";
import { input as inputs, output as outputs } from "../../types";
import * as utilities from "../../utilities";

import {ObjectMeta} from "../../meta/v1";

/**
 * CephRBDMirror represents a Ceph RBD Mirror
 */
export class CephRBDMirror extends pulumi.CustomResource {
    /**
     * Get an existing CephRBDMirror resource's state with the given name, ID, and optional extra
     * properties used to qualify the lookup.
     *
     * @param name The _unique_ name of the resulting resource.
     * @param id The _unique_ provider ID of the resource to lookup.
     * @param opts Optional settings to control the behavior of the CustomResource.
     */
    public static get(name: string, id: pulumi.Input<pulumi.ID>, opts?: pulumi.CustomResourceOptions): CephRBDMirror {
        return new CephRBDMirror(name, undefined as any, { ...opts, id: id });
    }

    /** @internal */
    public static readonly __pulumiType = 'kubernetes:ceph.rook.io/v1:CephRBDMirror';

    /**
     * Returns true if the given object is an instance of CephRBDMirror.  This is designed to work even
     * when multiple copies of the Pulumi SDK have been loaded into the same process.
     */
    public static isInstance(obj: any): obj is CephRBDMirror {
        if (obj === undefined || obj === null) {
            return false;
        }
        return obj['__pulumiType'] === CephRBDMirror.__pulumiType;
    }

    public readonly apiVersion!: pulumi.Output<"ceph.rook.io/v1" | undefined>;
    public readonly kind!: pulumi.Output<"CephRBDMirror" | undefined>;
    public readonly metadata!: pulumi.Output<ObjectMeta>;
    /**
     * RBDMirroringSpec represents the specification of an RBD mirror daemon
     */
    public readonly spec!: pulumi.Output<outputs.ceph.v1.CephRBDMirrorSpec>;
    /**
     * Status represents the status of an object
     */
    public readonly status!: pulumi.Output<{[key: string]: any} | undefined>;

    /**
     * Create a CephRBDMirror resource with the given unique name, arguments, and options.
     *
     * @param name The _unique_ name of the resource.
     * @param args The arguments to use to populate this resource's properties.
     * @param opts A bag of options that control this resource's behavior.
     */
    constructor(name: string, args?: CephRBDMirrorArgs, opts?: pulumi.CustomResourceOptions) {
        let resourceInputs: pulumi.Inputs = {};
        opts = opts || {};
        if (!opts.id) {
            resourceInputs["apiVersion"] = "ceph.rook.io/v1";
            resourceInputs["kind"] = "CephRBDMirror";
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
        super(CephRBDMirror.__pulumiType, name, resourceInputs, opts);
    }
}

/**
 * The set of arguments for constructing a CephRBDMirror resource.
 */
export interface CephRBDMirrorArgs {
    apiVersion?: pulumi.Input<"ceph.rook.io/v1">;
    kind?: pulumi.Input<"CephRBDMirror">;
    metadata?: pulumi.Input<ObjectMeta>;
    /**
     * RBDMirroringSpec represents the specification of an RBD mirror daemon
     */
    spec?: pulumi.Input<inputs.ceph.v1.CephRBDMirrorSpecArgs>;
    /**
     * Status represents the status of an object
     */
    status?: pulumi.Input<{[key: string]: any}>;
}
