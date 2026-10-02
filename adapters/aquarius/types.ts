```typescript
export interface SupplyConfig {
  reserveToken: `0x${string}`;
  borrowable: boolean;
  collateral: boolean;
  borrowableCollateral: boolean;
  interestRateStrategy: string;
  underlyingPriceFeedId: string;
  borrowedPriceFeedId: string;
  liquidityTarget: number;
  targetRatio: number;
  baseVariableBorrowRate: number;
  slope1BorrowRate: number;
  slope2BorrowRate: number;
  liquidityBonus: number;
  flashLoanFee: number;
  protocolSideFee: number;
  flashLoanFeeProtocolSide: number;
  supplyCap: number;
  totalCollateral: number;
  utilization: number;
  reserveFactor: number;
  pricePerShare: string;
  totalSupplyTokens: string;
  lastAccrualTime: number;
}

export interface DebtConfig {
  outstandingDebt: string;
  availableReserveBalance: string;
  lastUpdated: number;
  accruedProtocolRevenue: string;
  debtPriceFeedId: string;
  supplyBalance: string;
  reserveBalance: string;
  totalSupply: string;
  liquidity: string;
  lastAccrualTime: number;
  baseVariableBorrowRate: number;
  variableBorrowRate: number;
  reserveFactor: number;
  debtToken: `0x${string}`;
  supplyToken: `0x${string}`;
}

export interface PriceConfig {
  pricePerShare: string;
  reserveValue: string;
  marketValue: string;
}

export interface AquariusPoolInfo {
  poolAddress: string;
  config: SupplyConfig;
  debtConfig: DebtConfig;
  priceConfig: PriceConfig;
}
