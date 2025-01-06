# BurnTokensResponseResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction_plan** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Transaction details specific to the blockchain (e.g., userOp for ERC-4337, raw tx for EVM, instructions for Solana) | [optional]
**fee_breakdown** | Option<[**models::FeeBreakdown**](FeeBreakdown.md)> |  | [optional]
**burned_token** | Option<[**models::BurnTokensResponseResultBurnedToken**](BurnTokensResponse_result_burnedToken.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

