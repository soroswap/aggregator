```typescript
import { AquariusPoolInfo } from "./types";
import { AQUARIUS_ADAPTER, ADAPTER_BANK_FORM_FIELDS } from "./utils";

export function fillBankForm(poolInfo: AquariusPoolInfo): Record<string, string> {
  const result: Record<string, string> = {};
  for (const field of ADAPTER_BANK_FORM_FIELDS) {
    const parts = field.split(".");
    let value: unknown = poolInfo as Record<string, unknown>;
    for (const part of parts) {
      if (value && typeof value === "object") {
        value = (value as Record<string, unknown>)[part];
      } else {
        value = undefined;
        break;
      }
    }
    result[field] = value !== undefined && value !== null ? String(value) : "";
  }
  return result;
}

export function getAdapterAddress(): string {
  return AQUARIUS_ADAPTER;
}
