# \StakingApi

All URIs are relative to *https://api.emberai.xyz/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**stake_tokens_post**](StakingApi.md#stake_tokens_post) | **POST** /stake_tokens | Creates a new staking position.
[**unstake_tokens_post**](StakingApi.md#unstake_tokens_post) | **POST** /unstake_tokens | Unstakes tokens from a staking position.



## stake_tokens_post

> models::StakeTokensResponse stake_tokens_post(stake_tokens_request)
Creates a new staking position.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stake_tokens_request** | [**StakeTokensRequest**](StakeTokensRequest.md) | Stake tokens request payload. | [required] |

### Return type

[**models::StakeTokensResponse**](StakeTokensResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unstake_tokens_post

> models::UnstakeTokensResponse unstake_tokens_post(unstake_tokens_request)
Unstakes tokens from a staking position.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unstake_tokens_request** | [**UnstakeTokensRequest**](UnstakeTokensRequest.md) | Unstake tokens request payload. | [required] |

### Return type

[**models::UnstakeTokensResponse**](UnstakeTokensResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

