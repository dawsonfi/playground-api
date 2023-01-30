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
    ],
    "created_at": "dd/mm/yyyy",
    "updated_at": "dd/mm/yyyy"
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
    "month": "number",
    "year": "number",
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
    ],
    "created_at": "dd/mm/yyyy",
    "updated_at": "dd/mm/yyyy"
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
    ],
    "status": [
        "OPEN",
        "CLOSED"
    ],
    "created_at": "dd/mm/yyyy",
    "updated_at": "dd/mm/yyyy"
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

* POST `/transactions`
* POST `/transactions/id`

Request:

```json
{
    "source_id": "accounts:id",
    "destination_id": "accounts:id",
    "expense_id": "expenses:id",
    "description": "string",
    "date": "dd/mm/yyyy",
    "month": "number",
    "year": "number",
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

Reponse:

```json
{
    "id": "uuid",
}
```


### List Transactions

Request:
* GET `/transactions`
* GET `/transactions/id`
* GET `/transactions/expense_id`
* GET `/transactions/source_id`
* GET `/transactions/destination_id`
* GET `/transactions?operation=operation,type=type,status=status,month=month,year=year`

Response:

```json
[{
    "id": "uuid",
    "source_id": "accounts:id",
    "destination_id": "accounts:id",
    "expense_id": "expenses:id",
    "description": "string",
    "date": "dd/mm/yyyy",
    "month": "number",
    "year": "number",
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
}]
```

### Add Update Expenses

Request:

* POST `/expenses`
* POST `/expenses/id`

```json
{
    "description": "string",
    "date_added": "dd/mm/yyyy",
    "value": {
        "due_date": "dd/mm/yyyy",
        "added_date": "dd/mm/yyyy",
        "value": {
            "currency": ["BRL", "USD", "EUR", "..."],
            "value": "double",
        }        
    },
    "payment": {
        "due_date": "dd/mm/yyyy",
        "payment_date": "dd/mm/yyyy",
        "receive_date": "dd/mm/yyyy",
        "status": [
            "PAID",
            "UNPAID",
            "PLANNED",
            "CANCELLED"
        ]
    },
    "type": [
        "FIXED",
        "VARIABLE",
        "ONE_TIME",
        "LEND"
    ],
    "status": [
        "OPEN",
        "CLOSED"
    ]
}
```

Response:

```json
{
    "id": "uuid",
}
```

### List Expenses

Request:
* GET `/expenses`
* GET `/expenses/id`
* GET `/expenses?type=type,status=status`

Response:

```json
[{
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
    ],
    "status": [
        "OPEN",
        "CLOSED"
    ],
    "created_at": "dd/mm/yyyy",
    "updated_at": "dd/mm/yyyy"
}]
```

## Assumptions

* All endpoints must be idempotent, meaning that if the same transaction is performed more than one time, the subsequent request will not affect the values
* All endpoints must have a response time below 2 seconds
* All endpoints must have a 99.99% availability