from nuclear.img import NCLR
import json

backbeat = NCLR(open("test_files/backbeat.NCLR", "rb").read())
out = open("test_files/backbeat.NCLR.json", "w")
out.write(json.dumps(backbeat.serialize(), indent=4))
out.close()