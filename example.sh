#!/bin/sh

export ISSUER_URL=https://accounts.example.com

./rs-oidc-provider-metadata |
	dasel --read=json --write=toml --colour
