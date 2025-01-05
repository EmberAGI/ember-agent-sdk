# \DataApi

All URIs are relative to *https://api.emberai.xyz/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_supported_chains_get**](DataApi.md#get_all_supported_chains_get) | **GET** /get_all_supported_chains | Retrieves information about all supported blockchain networks.
[**get_all_supported_tokens_get**](DataApi.md#get_all_supported_tokens_get) | **GET** /get_all_supported_tokens | Retrieves a list of all supported tokens across all chains.
[**get_social_sentiment_get**](DataApi.md#get_social_sentiment_get) | **GET** /get_social_sentiment | Retrieves social sentiment data for a token or protocol.
[**get_token_data_get**](DataApi.md#get_token_data_get) | **GET** /get_token_data | Retrieves detailed information about a specific token.



## get_all_supported_chains_get

> models::GetAllSupportedChainsResponse get_all_supported_chains_get()
Retrieves information about all supported blockchain networks.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetAllSupportedChainsResponse**](GetAllSupportedChainsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_supported_tokens_get

> models::GetAllSupportedTokensResponse get_all_supported_tokens_get(chain_id, include_metadata)
Retrieves a list of all supported tokens across all chains.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain_id** | Option<**i32**> | Chain ID to filter tokens |  |
**include_metadata** | Option<**bool**> | Whether to include metadata |  |

### Return type

[**models::GetAllSupportedTokensResponse**](GetAllSupportedTokensResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_social_sentiment_get

> models::GetSocialSentimentResponse get_social_sentiment_get(identifier, r#type, timeframe)
Retrieves social sentiment data for a token or protocol.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Token address or protocol name | [required] |
**r#type** | **String** | Type of identifier | [required] |
**timeframe** | **String** | Timeframe for sentiment analysis | [required] |

### Return type

[**models::GetSocialSentimentResponse**](GetSocialSentimentResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_token_data_get

> models::GetTokenDataResponse get_token_data_get(chain_id, address)
Retrieves detailed information about a specific token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain_id** | **i32** | Chain ID of the token | [required] |
**address** | **String** | Address of the token | [required] |

### Return type

[**models::GetTokenDataResponse**](GetTokenDataResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

