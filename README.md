# Wahldaten Braunschweig / Election Data Brunswick

## REST API
There is a Rust `axum` based REST API server application. Swagger UI will be at `/docs/`, openapi.json at `/docs/openapi.json`

## Harvester
The harvester python script will query all open data endpoints of brunswick to collect election data from 2000 to today (missing the Bundestagswahl 2005 as not being available)
```
pip install -r harvester/requirements.txt
python harvester
```