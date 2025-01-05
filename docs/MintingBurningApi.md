# \MintingBurningApi

All URIs are relative to *https://api.emberai.xyz/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**burn_tokens_post**](MintingBurningApi.md#burn_tokens_post) | **POST** /burn_tokens | Creates an unsigned transaction payload for burning tokens.
[**mint_tokens_post**](MintingBurningApi.md#mint_tokens_post) | **POST** /mint_tokens | Creates an unsigned transaction payload for minting tokens.



## burn_tokens_post

> models::BurnTokensResponse burn_tokens_post(burn_tokens_request)
Creates an unsigned transaction payload for burning tokens.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**burn_tokens_request** | [**BurnTokensRequest**](BurnTokensRequest.md) | Burn tokens request payload. | [required] |

### Return type

[**models::BurnTokensResponse**](BurnTokensResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mint_tokens_post

> models::MintTokensResponse mint_tokens_post(mint_tokens_request)
Creates an unsigned transaction payload for minting tokens.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mint_tokens_request** | [**MintTokensRequest**](MintTokensRequest.md) | Mint tokens request payload. | [required] |

### Return type

[**models::MintTokensResponse**](MintTokensResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

