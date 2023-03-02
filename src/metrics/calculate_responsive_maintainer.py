import requests as rq
from pprint import pprint
import sys
import json
import datetime as dt
import os
from verbosity import setup_logging
import logging

devnull = open('/dev/null', 'w')
sys.stdout = devnull
sys.stderr = devnull

MAXNUMOPEN = 1000
UPDATEDECAY = 1.1

def getResponsiveScore(githubRepoURL):

    repoDir = githubRepoURL.split('https://github.com/')[1]

    # format each github url for the REST api
    # we want to get the number of open and closed issues in the repo

    openURL = 'https://api.github.com/repos/' + repoDir

    # Get the github token from the environment variables
    github_token = os.environ.get('GITHUB_TOKEN')

    # Setup logging
    setup_logging() 

    logging.info('>>> beginning respmaintainer metric with REST api')

    logging.info('current analysis of responsive maintainer metric will be done with github REST api')
    logging.info('------------------')
    logging.info('beginning retrieval of information from repository %s' % githubRepoURL)
    logging.info('------------------')


    headers = {'Authorization': 'token ' + github_token} # build the header for authentication
   
   # get a response using the REST API
    openResp = (rq.get(url=openURL,headers=headers))

    # if the response is successful, get the issue numbers
    if openResp.status_code == 200:
        logging.info('able to receive response code %d from github' % openResp.status_code)
        
        # test if the repos contain the correct data in json format
        try:
            respJson = openResp.json()

            hasIssues = respJson['has_issues']
            openNum = int(respJson['open_issues_count'])

            updatedDate = respJson['updated_at']
            updatedDate = updatedDate.split('-')
            year = int(updatedDate[0])
            month = int(updatedDate[1])
            day = int(updatedDate[2].split('T')[0])
            updatedDate = dt.date(year,month,day)

            logging.debug('proper repo format - retrieved %s from the api' % updatedDate)


        except:
            logging.error('improper repo format- investigate repo at %s' % githubRepoURL)
            return -1

        # calculate ratio of open to closed requests for score
        score = 0

        if hasIssues == True:
            logging.debug('repo had more than 0 issues = True')
            score += 0.05

        if openNum > 25:
            logging.debug('number of open issues exceeded threshold of 25')
            score += min(0.2,0.2 * ((MAXNUMOPEN - openNum) / MAXNUMOPEN))

        elapsedTime = str(dt.date.today() - updatedDate)
        if elapsedTime == '0:00:00':
            logging.debug('repo was updated today, i.e. day difference was %s' % elapsedTime)
            elapsedTime = 0
        else:
            elapsedTime = int(elapsedTime.split(' ')[0])
            logging.debug('repo was not updated today, it was updated %d days ago' % elapsedTime)
            if elapsedTime < 0:
                elapsedTime = 0

        score += 0.75 * (UPDATEDECAY ** (-1 * elapsedTime))

        logging.info('responsive maintainer score was calculated to be %f with decay factor %f' % (score,UPDATEDECAY))

        return score
    
    # return invalid score if not able to get repo information
    else:
        logging.error('failed to resolve repository with response code %d from github' % openResp.status_code)

        return -1


def getGithubURLs(repos):
  
    repositories = []
    for x in range(len(repos)): # extract owner and name of each repository
        repoName = repos[x].partition('github.com/')[2] # extract "owner/repo"

        if not repoName: # if github.com/ is not found, extract as npmjs package
            with open('output/cloned_repos/' + str(x+1) + '/package.json') as json_File:
                npmsRepo = json.load(json_File) # load json file containing repo info
                repoName = npmsRepo['repository'] # extract repo info

            if not isinstance(repoName, str): # if a dict is returned instead of str
                repoName = list(repoName.values())[1] # extract url from dict
                repoName = repoName.partition('github.com/')[2].replace('.git','') # extract "owner/repo"
    
        repositories.append((repoName.partition('/')[0],repoName.partition('/')[2].replace('\n',''))) # append (owner, repo)

    #convert all repos to github urls
    gitLinks = []

    for r in repositories:
        currLink = 'https://github.com/' + r[0] + '/' + r[1]
        gitLinks.append(currLink)

    #return list of github urls
    return gitLinks

def main():

    #read in data from test file
    testFile = open(sys.argv[1],'r')
    urls = testFile.readlines()

    #clear newlines from all urls
    for u in range(len(urls)):
        urls[u] = urls[u].replace('\n','')

    #convert any npm repos to github
    gitURLs = getGithubURLs(urls)

    #find and print the responsive maintainer metric for each repo
    with open('output/resp_maintain_out.txt', 'w') as f:
        for u in gitURLs:
            currScore = getResponsiveScore(u)
            #print('\nResponsive Maintainer score for repo: ', u, '\nis: ',currScore,'\n')
            f.write(str(currScore))
            f.write('\n')
if __name__ == "__main__":
    main()
