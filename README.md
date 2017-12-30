Architecture:

API is exposed through the user module, which further exposes how users interact with their bank accounts.

user module
- handles all user interactions with the system.

bank module
- exposes bank-specific limitations on the account module.

account module
- handles account arithmetic and operations.

