# **Tubbly 🤝 Casper Network**

### 📜 Smart Contract for Requesting Balance Change and Confirmation

This smart contract allows users to request changes to their balance
and requires confirmation by the contract owner before the changes can be executed.
It provides a secure and transparent way to manage balance adjustments within the contract.

### 🔒 Authority

The contract owner has the authority to approve or reject balance change requests made by users.
Only after confirmation by the contract owner will the requested changes be applied to the user's balance.

### 🚀 Features:

- Users can submit requests to increase or decrease their balance.
- Requests are pending until confirmed by the contract owner.
- The contract owner has the sole authority to confirm or reject balance change requests.
- Once confirmed, the requested balance changes are applied to the user's balance.
- Provides transparency and accountability in managing balance adjustments.

### 🔨 Usage:

1.  Users initiate a balance change request by calling the appropriate function with the desired amount.
2.  The request is added to the pending list until confirmed by the contract owner.
3.  The contract owner reviews pending requests and can confirm them.
4.  Confirmed requests are executed, and the user's balance is updated accordingly.

### 📝 Note

This contract assumes that the contract owner is a trusted entity with the authority to manage balance changes within the contract.
