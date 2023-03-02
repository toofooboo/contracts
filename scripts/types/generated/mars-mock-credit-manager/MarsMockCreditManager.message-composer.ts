// @ts-nocheck
/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.24.0.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

import { MsgExecuteContractEncodeObject } from 'cosmwasm'
import { MsgExecuteContract } from 'cosmjs-types/cosmwasm/wasm/v1/tx'
import { toUtf8 } from '@cosmjs/encoding'
import {
  InstantiateMsg,
  ExecuteMsg,
  Uint128,
  VaultPositionAmount,
  VaultAmount,
  VaultAmount1,
  UnlockingPositions,
  Addr,
  Decimal,
  Positions,
  DebtAmount,
  Coin,
  LentAmount,
  VaultPosition,
  LockingVaultAmount,
  VaultUnlockingPosition,
  VaultBaseForAddr,
  VaultConfig,
  QueryMsg,
  VaultBaseForString,
  ArrayOfCoinBalanceResponseItem,
  CoinBalanceResponseItem,
  ArrayOfSharesResponseItem,
  SharesResponseItem,
  ArrayOfDebtShares,
  DebtShares,
  ArrayOfLentShares,
  LentShares,
  ArrayOfVaultWithBalance,
  VaultWithBalance,
  ArrayOfVaultPositionResponseItem,
  VaultPositionResponseItem,
  ArrayOfString,
  ConfigResponse,
  ArrayOfCoin,
  VaultConfigResponse,
  VaultPositionValue,
  CoinValue,
  VaultUtilizationResponse,
  ArrayOfVaultConfigResponse,
} from './MarsMockCreditManager.types'
export interface MarsMockCreditManagerMessage {
  contractAddress: string
  sender: string
  setPositionsResponse: (
    {
      accountId,
      positions,
    }: {
      accountId: string
      positions: Positions
    },
    funds?: Coin[],
  ) => MsgExecuteContractEncodeObject
  setAllowedCoins: (funds?: Coin[]) => MsgExecuteContractEncodeObject
  setVaultConfig: (
    {
      address,
      config,
    }: {
      address: string
      config: VaultConfig
    },
    funds?: Coin[],
  ) => MsgExecuteContractEncodeObject
}
export class MarsMockCreditManagerMessageComposer implements MarsMockCreditManagerMessage {
  sender: string
  contractAddress: string

  constructor(sender: string, contractAddress: string) {
    this.sender = sender
    this.contractAddress = contractAddress
    this.setPositionsResponse = this.setPositionsResponse.bind(this)
    this.setAllowedCoins = this.setAllowedCoins.bind(this)
    this.setVaultConfig = this.setVaultConfig.bind(this)
  }

  setPositionsResponse = (
    {
      accountId,
      positions,
    }: {
      accountId: string
      positions: Positions
    },
    funds?: Coin[],
  ): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: '/cosmwasm.wasm.v1.MsgExecuteContract',
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            set_positions_response: {
              account_id: accountId,
              positions,
            },
          }),
        ),
        funds,
      }),
    }
  }
  setAllowedCoins = (funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: '/cosmwasm.wasm.v1.MsgExecuteContract',
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            set_allowed_coins: {},
          }),
        ),
        funds,
      }),
    }
  }
  setVaultConfig = (
    {
      address,
      config,
    }: {
      address: string
      config: VaultConfig
    },
    funds?: Coin[],
  ): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: '/cosmwasm.wasm.v1.MsgExecuteContract',
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            set_vault_config: {
              address,
              config,
            },
          }),
        ),
        funds,
      }),
    }
  }
}
