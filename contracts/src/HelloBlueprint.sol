// SPDX-License-Identifier: UNLICENSE
pragma solidity >=0.8.26;

import "tnt-core/BlueprintServiceManagerBase.sol";

/**
 * @title HelloBlueprint
 * @dev This contract is an example of a service blueprint that provides a single service.
 * @dev For all supported hooks, check the `BlueprintServiceManagerBase` contract.
 */
contract HelloBlueprint is BlueprintServiceManagerBase {
    /**
     * @dev Hook for service operator registration. Called when a service operator
     * attempts to register with the blueprint.
     * @param operator The operator's address.
     * @param registrationInputs Inputs required for registration in bytes format.
     */
    function onRegister(
        address operator,
        bytes calldata registrationInputs
    )
    external
    payable
    virtual
    override
    onlyFromTangle
    {
        // Do something with the operator's details
    }

    /**
     * @dev Hook for service instance requests. Called when a user requests a service
     * instance from the blueprint.
     */
    function onRequest(
        uint64 serviceId,
        address requester,
        address[] calldata operators,
        bytes calldata requestInputs,
        uint64 ttl,
        address paymentAsset,
        uint256 amount
    )
    external
    payable
    virtual
    override
    onlyFromTangle
    {
        // Do something with the service request
    }

    /**
     * @dev Hook for handling job result. Called when operators send the result
     * of a job execution.
     * @param serviceId The ID of the service related to the job.
     * @param job The job identifier.
     * @param jobCallId The unique ID for the job call.
     * @param operator The operator address sending the result.
     * @param inputs Inputs used for the job execution in bytes format.
     * @param outputs Outputs resulting from the job execution in bytes format.
     */
    function onJobResult(
        uint64 serviceId,
        uint8 job,
        uint64 jobCallId,
        address operator,
        bytes calldata inputs,
        bytes calldata outputs
    )
    external
    payable
    virtual
    override
    onlyFromTangle
    {
        // Do something with the job call result
    }
}
