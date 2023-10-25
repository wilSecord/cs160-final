#!/bin/bash

MAX_FILESIZE_BYTES=40M

mkdir data || (rm -rf data && mkdir data)

psql_user="$(sed -n '1p' data.txt)"
psql_host="$(sed -n '2p' data.txt)"
psql_port="$(sed -n '3p' data.txt)"
psql_dbse="$(sed -n '4p' data.txt)"
psql_pass="$(sed -n '5p' data.txt)"

function psql_exec() {
    PGPASSWORD="$psql_pass" psql -h "$psql_host" -p "$psql_port" -U "$psql_user" -d "$psql_dbse" --csv -f "$1"
}

psql_exec find_artists_area.pgsql

psql_exec find_collaborations.pgsql

# split large csv files into tinier ones in order to not fall afoul of the github size limit
find data -iname "*.csv" -size +"$MAX_FILESIZE_BYTES" \
    -exec mkdir "data/.split" \; \
    -exec split -d --line-bytes="$MAX_FILESIZE_BYTES" {} "data/.split/chunk_" \; \
    -exec mv "data/.split" "{}.split" \; \
    -exec rm {} \;