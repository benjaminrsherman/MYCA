import re
import requests
from bs4 import BeautifulSoup

##### GET COURSE IDS #####
id_rgx = re.compile("preview_course.*=.*=(\d+)")
coid_baseurl = "http://catalog.rpi.edu/content.php?navoid=444&filter[cpage]="

i = 1
coids = []
while True:
	catalog_page = requests.get(coid_baseurl + str(i))
	catalog_html = catalog_page.text
	current_coids = id_rgx.findall(catalog_html)	
	if len(current_coids) == 0:
		break
	coids.extend(current_coids)
	i += 1

coid_re = re.compile("[A-Z]{4} +\d{4}")
def parse_course(course_strings):
	course = {}

	course['complete'] = True # assume we parse correctly

	coid_and_name = course_strings[0].split("-")
	course['coid'] = {}
	course['coid']['subj'] = coid_and_name[0][:4]
	course['coid']['code'] = int(coid_and_name[0][5:-1])
	course['name'] = coid_and_name[1]
	course['description'] = course_strings[1]

	course['offered'] = ""
	course['prereqs'] = []
	course['prereqs_opt'] = []
	course['prereqs_one'] = []
	course['coreqs'] = []
	course['coreqs_opt'] = []
	course['coreqs_one'] = []

	i = 2
	in_prereqs = False
	while i < len(course_strings):
		if "When Offered:" == course_strings[i]:
			i += 1
			if i >= len(course_strings):
				break
			offered_lower = course_strings[i].lower()
			if "spring" in offered_lower:
				course['offered'] += "s"
			if "summer" in offered_lower:
				course['offered'] += "u"
			if "fall" in offered_lower:
				course['offered'] += "f"

			if "even" in offered_lower:
				course['offered'] += "e"
			if "odd" in offered_lower:
				course['offered'] += "o"
			in_prereqs = False
		elif in_prereqs or "Prerequisites/Corequisites:" == course_strings[i]:
			i += 1
			reqs = course_strings[i].split("orequisite") # no 'C' in case someone decides to make it lowercase
			prereqs = coid_re.findall(reqs[0])
			for prereq_str in prereqs:
				prereq_str_split = prereq_str.split(" ")
				prereq = {}
				prereq['subj'] = prereq_str_split[0]
				prereq['code'] = int(prereq_str_split[1])
				course['prereqs'].append(prereq)
			if len(reqs) > 1:
				coreqs = coid_re.findall(reqs[1])
				for coreq_str in coreqs:
					coreq_str_split = coreq_str.split(" ")
					coreq = {}
					coreq['subj'] = coreq_str_split[0]
					coreq['code'] = int(coreq_str_split[1])
					course['coreqs'].append(coreq)
			in_prereqs = True
				
		if not in_prereqs:
			i += 1		
	
	course['post_options'] = []

	return course

courses = {}
courses['courses'] = []

##### GET COURSE INFO #####
course_baseurl = "http://catalog.rpi.edu/preview_course.php?coid="
for coid in coids:
	course_url = course_baseurl + coid
	course_html = requests.get(course_url).text
	entry = BeautifulSoup(course_html, "html.parser").find("td", class_="block_content_popup")
	title = entry.find("h1")
	course_strings = [text for text in entry.stripped_strings][4:-5]
	try:
		course = parse_course(course_strings)
		courses['courses'].append(course)
		print("Parsed: " + course['coid']['subj'] + " " + str(course['coid']['code']))
	except:
		continue

import json
with open('catalog.json', 'w') as outfile:
	json.dump(courses, outfile, indent=2)
