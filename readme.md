## CUSTOM MIGRATION COMMAND:

```
cargo run --bin rust_test -- migrate
```
## SQLX OFFLINE MODE:

```
cargo sqlx prepare
```

## ENVIRON VARIABLE 
- Set the following environ variables in files called `env.sh` and `configuration.yaml`.
- The value in `env.sh` are:
```

## DATABASE VARIABLES
export DATABASE__PASSWORD=""
export DATABASE__PORT=00
export DATABASE__HOST=""
export DATABASE__NAME=""
export DATABASE__USERNAME="postgres"
export DATABASE__DATABASE_URL="postgres://postgres:{password}@{host}:{port}/{db_name}"

## EMAIL VARIABLES
export EMAIL_CLIENT__USERNAME=""
export EMAIL_CLIENT__PASSWORD=""
export EMAIL_CLIENT__BASE_URL=""
export EMAIL_CLIENT__SENDER_EMAIL=""
export EMAIL_CLIENT__TIMEOUT_MILLISECONDS=10000


## TARACING VARIABLES
export OTEL_SERVICE_NAME="rust_test"
export OTEL_EXPORTER_OTLP_TRACES_ENDPOINT="http://localhost:4317"
# export OTEL_EXPORTER_OTLP_ENDPOINT="http://localhost:4318/v1/traces"
# export OTEL_INSTRUMENTATION_HTTP_CAPTURE_HEADERS_SERVER_REQUEST="X-Request-*"

## LOG VARIABLES
export TEST_LOG=True

## SECRET VALUES
export SECRET__JWT__SECRET=""

## REDIS VARIABLE
export REDIS__HOST=""
export REDIS__PORT=""
export REDIS_PASSWORD=""


## BUYER NP DETAILS
export ONDC__BAP__ID=""
export ONDC__BAP__URI=""
export ONDC__BAP__SIGNING_KEY=""

##USER VARIABLE
export LIST__USER__ADMIN_LIST="9562279968,"

## ONDC GATEWAY VARIABLE
export ONDC__GATEWAY_URI="https://preprod.gateway.ondc.org"
export ONDC__REGISTRY_BASE_URL="https://preprod.registry.ondc.org/ondc"

## PRODUCT MEILEI
export PRODUCT__MEILEI__API_MASTER_KEY=""
export PRODUCT__MEIELI__URL=""

```

- The value in `configuration.yaml` are:

```
application:
  port: 8002
  host: 0.0.0.0

```

- In order to verify SQL queries at compile time, set the below config in `.env` file:
```
export DATABASE_URL="postgres://postgres:{password}@{host}:{port}/{db_name}"

```

## TO RUN THE SERVER:
- For running development server:
```
bash dev_run.sh
```
- For running production server:
```
bash release.sh
```


## MILESTONE 1:
* [x] SET UP THE BASIC ACTIX SERVER
* [x] ADD AUTOMATIC CONFIGURATION FETCH
* [x] ADD ERROR HANDLING
* [x] DEVELOP CUSTOM MIGRATION COMMAND
* [x] ADD TRACING WITH JAEGER INTEGRATION
* [x] DEVELOP `SAVE REQ/RESPONSE PAYLOAD IN TRACING` MIDDLEWARE
* [x] EMAIL SERVICE INTEGRATION
* [x] DEVELOP USER REGISTRATION API
* [x] DEVELOP BUSINESS REGISTRATION API
* [x] DEVEOP USER AUTHENTICATION VIA PASSWORD
* [x] SETTING THE CODEBASE STRUCTURE SIMILAR TO DJANGO
* [x] DEVELOP JWT CREATION API AND VERIFICATION MIDDLEWARE
* [x] INTEGRATE OPENAPI SWAGGER
* [x] INTEGRATE REDIS
* [x] WEBSOCKET INTEGRATION
* [x] ADD BUSINESS VERIFICATION MIDDLEWARE
* [x] ADD TSP CHANGES INTO USER ACCOUNT AND BUSINESS ACCOUNT
* [x] ADD HEADER VALIDATION MIDDLEWARE
* [ ] COMPLETE REALTIME SEARCH API + ONDC SEARCH API (/search + /on_search)
* [ ] FIX TEST INTEGRATION + ADD TEST CASES



## MILESTONE 2:
* [ ] INTEGRATE PERMISSION FLOW
* [ ] COMPLETE VALIDATION FOR BUSINESS_ACCOUNT REGISTRATION



## MILESTONE 3:


## OPTIONAL
* [ ] DEVELOP NEW CONFIG FETCH (will be given the last priority)
* [ ] INTEGRATE SMS
* [ ] INTEGRATE WHATSAPP
* [ ] DEVELOP EMAIL VERFICATION APIS FOR USER AND BUSINESS ACCOUNT
* [ ] DEVELOP MOBILE VERFICATION APIS FOR USER AND BUSINESS ACCOUNT
* [ ] DEVELOP SMS OTP API
* [ ] DEVELOP EMAIL OTP API
* [ ] CREATE DOCKER IMAGE CREATION FILE
* [ ] INTERGRATE ETCD FOR TSP FLOW (WHEN I HAVE NOTHING BETTER TO DO: PROBABLY NEVER)
### THE MILESTONES ARE SUSCEPTIBLE TO CHANGES ╰(*°▽°*)╯