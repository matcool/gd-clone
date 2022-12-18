import xml.etree.ElementTree as ElementTree
import sys
import base64
import gzip

file_name = sys.argv[1]
output = sys.argv[2]

tree = ElementTree.parse(file_name)
root = tree.getroot()
children = list(root)
for i, child in enumerate(children):
	if child.tag == 'k' and child.text == 'k4':
		break

level_string = children[i + 1].text
level_string = gzip.decompress(base64.urlsafe_b64decode(level_string)).decode()
# header, objects = level_string.split(';', 1)
with open(output, 'w') as file:
	file.write(level_string)
