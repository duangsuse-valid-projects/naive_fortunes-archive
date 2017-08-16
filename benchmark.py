#!/usr/bin/python3
from sys import argv
import time
import urllib.request
from urllib.error import HTTPError
import string

TIMES=1000
PROTO="http"
#HOST="localhost"
HOST="0.0.0.0"
#PORT=8000
PORT=80
REQ="/fortune" if len(argv)!=2 else argv[1]

time_start=time.time()
for i in range(TIMES):
    try:
        r=urllib.request.urlopen("{}://{}:{}{}".format(PROTO, HOST, PORT, REQ))
        good=True
    except HTTPError as error:
        print("failed to request. HTTP {} message:{} fp:{}".format(error.code, error.msg, error.fp.read().decode()))
        good=False
    if good:
        print(r.read().decode())
time_dur=time.time()-time_start
print("requesting {} for {} times finished in {} secs".format(HOST, TIMES, time_dur))
