# Architecture Document

## Problem

This project is intended to be a playground project to experiment with Rust tools, api, CI/CD and keep an updated reference project.

The architecture is based and deployed on [AWS](https://aws.amazon.com/).

## Solution

For the problem, i'm creating a simple api to manage montly expenses and account balance, is not meant to be robust with finacial features, just enough to be simple and leverage the tools used for experimentation.

## Diagram

![Architecture Diagram](/dev/architecture//playground_api.drawio.png?raw=true)

## Database

### Accounts

This database will hold the accounts data, including their balance at the end of each month

Schema:

```json
{
    "id": "uuid",
    "name": "string",
    "bank_name": "string",
    "open_date": "dd/mm/yyyy",
    "close_date": "dd/mm/yyyy",
    "type": [
        "SALARY",
        "SAVINGS",
        "CHECKING",
        "INVESTMENT",
        "STOCK",
        "EXTERNAL_PARTY"
    ],
    "balances": [{
        "date": "dd/mm/yyyy",
        "balance": {
            "currency": ["BRL", "USD", "EUR", "..."],
            "value": "double"
        }
    }],
    "status": [
        "OPEN",
        "CLOSED",
        "NOT_IN_USE"
    ]
}
```

### Transactions

This database will hold records of each transaction made, adding or removing balance to the accounts, paying expenses, receiving money from external parties, etc.

Schema:

```json
{
    "id": "uuid",
    "source_id": "accounts:id",
    "destination_id": "accounts:id",
    "expense_id": "expenses:id",
    "description": "string",
    "date": "dd/mm/yyyy",
    "value": {
        "currency": ["BRL", "USD", "EUR", "..."],
        "value": "double"
    },
    "operation": [
        "CREDIT",
        "DEBIT"
    ],
    "type": [
        "SALARY",
        "PAYMENT",
        "TRANSFER"
    ],
    "status": [
        "OPEN",
        "CLOSED"
    ]
}
```

### Expenses

This database will hold the montly expenses, that being fixed, variable or one-time.

Schema:

```json
{
    "id": "uuid",
    "description": "string",
    "date_added": "dd/mm/yyyy",    
    "value_history": [{
        "due_date": "dd/mm/yyyy",
        "added_date": "dd/mm/yyyy",
        "value": {
            "currency": ["BRL", "USD", "EUR", "..."],
            "value": "double",
        }        
    }],
    "payments": [{
        "due_date": "dd/mm/yyyy",
        "payment_date": "dd/mm/yyyy",
        "receive_date": "dd/mm/yyyy",
        "status": [
            "PAID",
            "UNPAID",
            "PLANNED",
            "CANCELLED"
        ]
    }],
    "type": [
        "FIXED",
        "VARIABLE",
        "ONE_TIME",
        "LEND"
    ]
}
```

## Apis

### Add/Update Account

Adds or update an account on the dabatase:

Request:

* POST `/accounts`
* POST `/accounts/id`

```json
{    
    "name": "string",
    "bank_name": "string",
    "open_date": "dd/mm/yyyy",
    "close_date": "dd/mm/yyyy",
    "type": [
        "SALARY",
        "SAVINGS",
        "CHECKING",
        "INVESTMENT",
        "STOCK",
        "EXTERNAL_PARTY"
    ],
    "status": [
        "OPEN",
        "CLOSED",
        "NOT_IN_USE"
    ]
}
```

Response:

```json
{
    "id": "uuid",    
}
```

### List Accounts

List all accounts on the database based on filter criteria

Requests:

* GET `/accounts`
* GET `/accounts/id`
* GET `/accounts?type=type,status=status`

Respose:
```json
[{
    "id": "uuid",
    "name": "string",
    "bank_name": "string",
    "open_date": "dd/mm/yyyy",
    "close_date": "dd/mm/yyyy",
    "type": [
        "SALARY",
        "SAVINGS",
        "CHECKING",
        "INVESTMENT",
        "STOCK",
        "EXTERNAL_PARTY"
    ],
    "balances": [{
        "date": "dd/mm/yyyy",
        "balance": {
            "currency": ["BRL", "USD", "EUR", "..."],
            "value": "double"
        }
    }],
    "status": [
        "OPEN",
        "CLOSED",
        "NOT_IN_USE"
    ]
}]
```


### Add/Update Transaction

TODO

### List Transactions

TODO

### Add Update Expenses

TODO

### List Expenses

TODO