# Wahldaten Braunschweig / Election Data Brunswick

## Dashboard
Interactive Svelte App for visualization of data and prediction of election results.

To start:
```bash
cd dashboard
npm run dev
```


## REST API
There is a Rust `axum` based REST API server application. Swagger UI will be at `/docs/`, openapi.json at `/docs/openapi.json`. The API is live at https://wahlapi.jenskrumsieck.de.

## Harvester
The harvester python script will query all open data endpoints of Brunswick to collect election data from 2000 to today (missing the Bundestagswahl 2005 as not being available)
```bash
pip install -r harvester/requirements.txt
python harvester
```
Data will be harvested from [Votemanager](https://votemanager.kdo.de/03101000/index.html) and [Wahlatlas](https://www3.braunschweig.de/statistik/2025_Wahl-Atlas/atlas.html) portals of Brunswick