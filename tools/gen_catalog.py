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

coid_re = re.compile("[A-Z]{4}\s+\d{4}")
def parse_course(course_strings):
	if len(course_strings) == 0:
		return None
		
	course = {}

	course['complete'] = True # assume we parse correctly

	coid_and_name = course_strings[0].split(" - ")
	if len(coid_and_name[0]) > 9:
		return None

	course['coid'] = {}
	course['coid']['subj'] = coid_and_name[0][:4]
	course['coid']['code'] = int(coid_and_name[0][-4:])
	course['name'] = coid_and_name[1]
	course['description'] = ""

	course['offered'] = ""
	course['age_reqs'] = ""
	course['prereqs'] = []
	course['prereqs_opt'] = []
	course['coreqs'] = []
	course['coreqs_opt'] = []
	
	course['post_options'] = []

	if len(course_strings) == 1:
		return course
	course['description'] = course_strings[1]

	i = 2
	in_prereqs = False
	reqs_str = ""
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
			if i == len(course_strings):
				break				
			reqs_str += course_strings[i]
			in_prereqs = True
				
		if not in_prereqs:
			i += 1		
	
	if "freshman" in reqs_str.lower():
		course['age_reqs'] += "f"
	if "sophomore" in reqs_str.lower():
		course['age_reqs'] += "o"
	if "junior" in reqs_str.lower():
		course['age_reqs'] += "j"
	if "senior" in reqs_str.lower():
		course['age_reqs'] += "s"
	if "graduate" in reqs_str.lower():
		course['age_reqs'] += "g"
	
	reqs = reqs_str.split("orequisite") # no 'C' in case someone decides to make it lowercase
	for match in reqs[0].split(" and "):
		req_set = []
		match_courses = coid_re.findall(match)
		for req_str in match_courses:
			req = {}
			req['subj'] = req_str[:4]
			req['code'] = int(req_str[-4:])
			if " or " in match and " permission " not in match:
				req_set.append(req)
			else:
				course['prereqs'].append([req])
		if " or " in match and " permission " not in match:
			course['prereqs'].append(req_set)

	if len(reqs) > 1:
		for match in reqs[1].split(" and "):
			req_set = []
			match_courses = coid_re.findall(match)
			for req_str in match_courses:
				req = {}
				req['subj'] = req_str[:4]
				req['code'] = int(req_str[-4:])
				if " or " in match and " permission " not in match:
					req_set.append(req)
				else:
					course['coreqs'].append([req])
			if " or " in match and " permission " not in match:
				course['coreqs'].append(req_set)

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
	course = parse_course(course_strings)
	if course is None:
		continue
	courses['courses'].append(course)
	print("Parsed: " + course['coid']['subj'] + " " + str(course['coid']['code']))

import json
with open('catalog.json', 'w') as outfile:
	json.dump(courses, outfile, indent=2)
