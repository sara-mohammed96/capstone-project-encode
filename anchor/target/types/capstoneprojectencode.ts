/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/capstoneprojectencode.json`.
 */
export type Capstoneprojectencode = {
  "address": "5rwPBBSHeHrhBAN7PCDxg7aqA1zrS4PzZS7Mg3b5ainm",
  "metadata": {
    "name": "capstoneprojectencode",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "close",
      "discriminator": [
        98,
        165,
        201,
        177,
        108,
        65,
        206,
        96
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "capstoneprojectencode",
          "writable": true
        }
      ],
      "args": []
    },
    {
      "name": "fetchValidators",
      "discriminator": [
        92,
        233,
        241,
        145,
        93,
        204,
        208,
        223
      ],
      "accounts": [
        {
          "name": "voteAccounts",
          "docs": [
            "A generic account list to represent vote accounts"
          ],
          "address": "Vote111111111111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "initialize",
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "capstoneprojectencode",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "capstoneprojectencode",
      "discriminator": [
        168,
        65,
        241,
        144,
        89,
        112,
        38,
        31
      ]
    }
  ],
  "types": [
    {
      "name": "capstoneprojectencode",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "count",
            "type": "u8"
          }
        ]
      }
    }
  ]
};
