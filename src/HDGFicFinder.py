import AO3
import random

def getRandomHDG():
    pageNum = random.randint(0,3)
    print(pageNum)
    search = AO3.Search(fandoms="Human Domestication Guide - GlitchyRobo", page = pageNum, tags="little sprout")
    search.update()
    print(search.total_results)
    work = random.choice(search.results)
    workString = (work.title + " by ")
    for author in work.authors:
        workString = workString + author.username + " "
    return workString


