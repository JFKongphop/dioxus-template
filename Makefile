serve:
	serve -s dist/public

bl:
	dx bundle --platform web --out-dir dist

dx:
	dx serve --port 5173

### INTERNAL COMMAND
BASH := ./bash-request/

tt:
	sh ${BASH}total.sh ${year}

md:
	sh ${BASH}monthly-distance.sh ${year}

mdd: 
	sh ${BASH}month-day-distance.sh ${year}

tpd:
	sh ${BASH}total-program-distance.sh ${dist} ${year} 

pd:
	sh ${BASH}program-distance.sh ${dist} ${year}

ad:
	sh ${BASH}activity-distance.sh ${atv}

mad:
	sh ${BASH}month-activity-distance.sh ${year}

pg:
	sh ${BASH}pace-group.sh ${year}