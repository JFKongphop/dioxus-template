serve:
	serve -s dist/public

bl:
	dx bundle --platform web --out-dir dist

dx:
	dx serve --port 5173