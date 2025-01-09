# ByteMolar API Documentation

## Overview
This document outlines the API endpoints and interactions available in the ByteMolar platform.

## Solana Program Instructions

### Create Clinic
Creates a new dental clinic account.

```typescript
createClinic(
  authority: PublicKey,
  name: string
): Promise<TransactionSignature>