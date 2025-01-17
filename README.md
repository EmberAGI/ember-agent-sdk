# Rust API client for openapi

Comprehensive specifications for all Ember API endpoints, incorporating the complete set of DeFi capabilities.


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.2
- Package version: 0.2
- Generator version: 7.10.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.emberai.xyz/v1*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*BorrowingApi* | [**borrow_tokens_post**](docs/BorrowingApi.md#borrow_tokens_post) | **POST** /borrow_tokens | Creates a new borrowing position using deposited collateral.
*BorrowingApi* | [**repay_borrowed_tokens_post**](docs/BorrowingApi.md#repay_borrowed_tokens_post) | **POST** /repay_borrowed_tokens | Repays borrowed assets.
*DataApi* | [**get_all_supported_chains_get**](docs/DataApi.md#get_all_supported_chains_get) | **GET** /get_all_supported_chains | Retrieves information about all supported blockchain networks.
*DataApi* | [**get_all_supported_tokens_get**](docs/DataApi.md#get_all_supported_tokens_get) | **GET** /get_all_supported_tokens | Retrieves a list of all supported tokens across all chains.
*DataApi* | [**get_social_sentiment_get**](docs/DataApi.md#get_social_sentiment_get) | **GET** /get_social_sentiment | Retrieves social sentiment data for a token or protocol.
*DataApi* | [**get_token_data_get**](docs/DataApi.md#get_token_data_get) | **GET** /get_token_data | Retrieves detailed information about a specific token.
*LendingApi* | [**lend_tokens_post**](docs/LendingApi.md#lend_tokens_post) | **POST** /lend_tokens | Supplies assets to a lending protocol.
*LendingApi* | [**withdraw_lent_tokens_post**](docs/LendingApi.md#withdraw_lent_tokens_post) | **POST** /withdraw_lent_tokens | Withdraws assets from a lending position.
*LiquidityApi* | [**add_liquidity_post**](docs/LiquidityApi.md#add_liquidity_post) | **POST** /add_liquidity | Creates a new liquidity position in a DEX pool.
*LiquidityApi* | [**remove_liquidity_post**](docs/LiquidityApi.md#remove_liquidity_post) | **POST** /remove_liquidity | Removes liquidity from a DEX pool position.
*MintingBurningApi* | [**burn_tokens_post**](docs/MintingBurningApi.md#burn_tokens_post) | **POST** /burn_tokens | Creates an unsigned transaction payload for burning tokens.
*MintingBurningApi* | [**mint_tokens_post**](docs/MintingBurningApi.md#mint_tokens_post) | **POST** /mint_tokens | Creates an unsigned transaction payload for minting tokens.
*StakingApi* | [**stake_tokens_post**](docs/StakingApi.md#stake_tokens_post) | **POST** /stake_tokens | Creates a new staking position.
*StakingApi* | [**unstake_tokens_post**](docs/StakingApi.md#unstake_tokens_post) | **POST** /unstake_tokens | Unstakes tokens from a staking position.
*SwapsApi* | [**create_limit_order_post**](docs/SwapsApi.md#create_limit_order_post) | **POST** /create_limit_order | Creates a limit order for token swaps.
*SwapsApi* | [**create_swap_post**](docs/SwapsApi.md#create_swap_post) | **POST** /create_swap | Creates an unsigned transaction payload for swapping tokens.
*TransactionsApi* | [**submit_transaction_post**](docs/TransactionsApi.md#submit_transaction_post) | **POST** /submit_transaction | Submits a signed transaction to the network.
*TransfersApi* | [**create_transfer_post**](docs/TransfersApi.md#create_transfer_post) | **POST** /create_transfer | Creates an unsigned transaction payload for sending tokens.


## Documentation For Models

 - [AddLiquidityRequest](docs/AddLiquidityRequest.md)
 - [AddLiquidityRequestToken0](docs/AddLiquidityRequestToken0.md)
 - [AddLiquidityRequestToken1](docs/AddLiquidityRequestToken1.md)
 - [AddLiquidityResponse](docs/AddLiquidityResponse.md)
 - [AddLiquidityResponseResult](docs/AddLiquidityResponseResult.md)
 - [AddLiquidityResponseResultPositionDetails](docs/AddLiquidityResponseResultPositionDetails.md)
 - [BorrowTokensRequest](docs/BorrowTokensRequest.md)
 - [BorrowTokensResponse](docs/BorrowTokensResponse.md)
 - [BorrowTokensResponseResult](docs/BorrowTokensResponseResult.md)
 - [BurnTokensRequest](docs/BurnTokensRequest.md)
 - [BurnTokensResponse](docs/BurnTokensResponse.md)
 - [BurnTokensResponseResult](docs/BurnTokensResponseResult.md)
 - [BurnTokensResponseResultBurnedToken](docs/BurnTokensResponseResultBurnedToken.md)
 - [CreateLimitOrderRequest](docs/CreateLimitOrderRequest.md)
 - [CreateLimitOrderRequestBuyToken](docs/CreateLimitOrderRequestBuyToken.md)
 - [CreateLimitOrderResponse](docs/CreateLimitOrderResponse.md)
 - [CreateLimitOrderResponseResult](docs/CreateLimitOrderResponseResult.md)
 - [CreateSwapRequest](docs/CreateSwapRequest.md)
 - [CreateSwapRequestSellToken](docs/CreateSwapRequestSellToken.md)
 - [CreateSwapResponse](docs/CreateSwapResponse.md)
 - [CreateSwapResponseResult](docs/CreateSwapResponseResult.md)
 - [CreateSwapResponseResultEstimatedOutput](docs/CreateSwapResponseResultEstimatedOutput.md)
 - [CreateTransferRequest](docs/CreateTransferRequest.md)
 - [CreateTransferResponse](docs/CreateTransferResponse.md)
 - [CreateTransferResponseResult](docs/CreateTransferResponseResult.md)
 - [FeeBreakdown](docs/FeeBreakdown.md)
 - [GetAllSupportedChainsResponse](docs/GetAllSupportedChainsResponse.md)
 - [GetAllSupportedChainsResponseResult](docs/GetAllSupportedChainsResponseResult.md)
 - [GetAllSupportedChainsResponseResultChainsInner](docs/GetAllSupportedChainsResponseResultChainsInner.md)
 - [GetAllSupportedChainsResponseResultChainsInnerNativeCurrency](docs/GetAllSupportedChainsResponseResultChainsInnerNativeCurrency.md)
 - [GetAllSupportedTokensResponse](docs/GetAllSupportedTokensResponse.md)
 - [GetAllSupportedTokensResponseResult](docs/GetAllSupportedTokensResponseResult.md)
 - [GetAllSupportedTokensResponseResultPagination](docs/GetAllSupportedTokensResponseResultPagination.md)
 - [GetAllSupportedTokensResponseResultTokensInner](docs/GetAllSupportedTokensResponseResultTokensInner.md)
 - [GetSocialSentimentResponse](docs/GetSocialSentimentResponse.md)
 - [GetSocialSentimentResponseResult](docs/GetSocialSentimentResponseResult.md)
 - [GetSocialSentimentResponseResultSentiment](docs/GetSocialSentimentResponseResultSentiment.md)
 - [GetSocialSentimentResponseResultSentimentSources](docs/GetSocialSentimentResponseResultSentimentSources.md)
 - [GetSocialSentimentResponseResultSentimentSourcesDiscord](docs/GetSocialSentimentResponseResultSentimentSourcesDiscord.md)
 - [GetSocialSentimentResponseResultSentimentSourcesReddit](docs/GetSocialSentimentResponseResultSentimentSourcesReddit.md)
 - [GetSocialSentimentResponseResultSentimentSourcesTwitter](docs/GetSocialSentimentResponseResultSentimentSourcesTwitter.md)
 - [GetTokenDataResponse](docs/GetTokenDataResponse.md)
 - [GetTokenDataResponseResult](docs/GetTokenDataResponseResult.md)
 - [LendTokensRequest](docs/LendTokensRequest.md)
 - [LendTokensResponse](docs/LendTokensResponse.md)
 - [LendTokensResponseResult](docs/LendTokensResponseResult.md)
 - [LendTokensResponseResultPositionDetails](docs/LendTokensResponseResultPositionDetails.md)
 - [LendTokensResponseResultPositionDetailsAToken](docs/LendTokensResponseResultPositionDetailsAToken.md)
 - [LendTokensResponseResultPositionDetailsSuppliedToken](docs/LendTokensResponseResultPositionDetailsSuppliedToken.md)
 - [MintTokensRequest](docs/MintTokensRequest.md)
 - [MintTokensResponse](docs/MintTokensResponse.md)
 - [MintTokensResponseResult](docs/MintTokensResponseResult.md)
 - [MintTokensResponseResultMintedToken](docs/MintTokensResponseResultMintedToken.md)
 - [OrderDetails](docs/OrderDetails.md)
 - [OrderDetailsSellToken](docs/OrderDetailsSellToken.md)
 - [RemoveLiquidityRequest](docs/RemoveLiquidityRequest.md)
 - [RemoveLiquidityResponse](docs/RemoveLiquidityResponse.md)
 - [RemoveLiquidityResponseResult](docs/RemoveLiquidityResponseResult.md)
 - [RepayBorrowedTokensRequest](docs/RepayBorrowedTokensRequest.md)
 - [RepayBorrowedTokensResponse](docs/RepayBorrowedTokensResponse.md)
 - [RepayBorrowedTokensResponseResult](docs/RepayBorrowedTokensResponseResult.md)
 - [RepayBorrowedTokensResponseResultRepaymentDetails](docs/RepayBorrowedTokensResponseResultRepaymentDetails.md)
 - [StakeTokensRequest](docs/StakeTokensRequest.md)
 - [StakeTokensResponse](docs/StakeTokensResponse.md)
 - [StakeTokensResponseResult](docs/StakeTokensResponseResult.md)
 - [StakeTokensResponseResultStakingPosition](docs/StakeTokensResponseResultStakingPosition.md)
 - [StandardError](docs/StandardError.md)
 - [StandardErrorError](docs/StandardErrorError.md)
 - [SubmitTransactionRequest](docs/SubmitTransactionRequest.md)
 - [SubmitTransactionResponse](docs/SubmitTransactionResponse.md)
 - [SubmitTransactionResponseResult](docs/SubmitTransactionResponseResult.md)
 - [UnstakeTokensRequest](docs/UnstakeTokensRequest.md)
 - [UnstakeTokensResponse](docs/UnstakeTokensResponse.md)
 - [UnstakeTokensResponseResult](docs/UnstakeTokensResponseResult.md)
 - [UnstakeTokensResponseResultUnstakeDetails](docs/UnstakeTokensResponseResultUnstakeDetails.md)
 - [ValidationError](docs/ValidationError.md)
 - [ValidationErrorError](docs/ValidationErrorError.md)
 - [ValidationErrorErrorValidationErrorsInner](docs/ValidationErrorErrorValidationErrorsInner.md)
 - [WithdrawLentTokensRequest](docs/WithdrawLentTokensRequest.md)
 - [WithdrawLentTokensResponse](docs/WithdrawLentTokensResponse.md)
 - [WithdrawLentTokensResponseResult](docs/WithdrawLentTokensResponseResult.md)
 - [WithdrawLentTokensResponseResultWithdrawalDetails](docs/WithdrawLentTokensResponseResultWithdrawalDetails.md)
 - [WithdrawLentTokensResponseResultWithdrawalDetailsRemainingPosition](docs/WithdrawLentTokensResponseResultWithdrawalDetailsRemainingPosition.md)
 - [WithdrawLentTokensResponseResultWithdrawalDetailsWithdrawnToken](docs/WithdrawLentTokensResponseResultWithdrawalDetailsWithdrawnToken.md)
 - [WithdrawalDetails](docs/WithdrawalDetails.md)
 - [WithdrawalDetailsCollectedFees](docs/WithdrawalDetailsCollectedFees.md)
 - [WithdrawalDetailsToken0](docs/WithdrawalDetailsToken0.md)
 - [WithdrawalDetailsToken1](docs/WithdrawalDetailsToken1.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



