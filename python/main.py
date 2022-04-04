from nuclear.img import NCLR
import json
import os

backbeat = NCLR(open("test_files/backbeat.NCLR", "rb").read())

imgs = backbeat.export()

for file in imgs:
    try:
        os.mkdir("test_files/backbeat.NCLR-out")
    except:
        pass
    out = open(f"test_files/backbeat.NCLR-out/{file}", "wb")
    out.write(imgs[file])
    out.close()

#out = open("test_files/backbeat.NCLR.json", "w")
#out.write(json.dumps(backbeat.serialize(), indent=4))
#out.close()