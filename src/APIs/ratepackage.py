'''
This file contains the following api call:
    - /package/<int:id>/rate (GET)

    **Assumes server is started from project_461 directory**
    **Until authentication token is provided, this will return a 400**

'''

from flask import jsonify
import subprocess
import json
import datetime
from flask.blueprints import Blueprint
from .database import db_connect 
from .auth import *
from sqlalchemy.sql import text
import os

bp = Blueprint('ratepackage', __name__)


# Rates a package if it exists and stores the rating in PackageRating table
@bp.route('/package/<int:id>/rate', methods=['GET'])
#@token_required
def rate_package(id):
    cnx = db_connect()

    # Get the package url from the database
    package_url = get_package_url(id, cnx)

    # If the package doesn't exist, return a 404
    if package_url is None:
        print("package does not exist, returning 404")
        return "Package {} not found".format(id), 404
    else:
        package_url = package_url[0]

    # Set the relative path to the CLI from the directory the server was started from **update this**
    # Currently assumes the server is started from the project_461 directory
    clipath = "./run"
    result = ""

    try:
        # ./run "package_url" from and return the results
        # rating = run_cli(package_url, clipath)
        rating = subprocess.check_output([clipath, package_url], env=os.environ)
        print(f"Rating json: {rating}")
        result = rating.decode("utf-8")
    except Exception as e:
        # If the rating returns an error, return a 500
        return "Error: rating failed for package {} with error [{}]".format(id, e), 500
    else:
        if len(result) < 174:
            return "Error: Could not get rating for package {} with result {}".format(id, result), 500
        rating = json.loads(rating)

    # Insert the results into the database
    # See if ID is already in PackageRating
    package_rating = get_package_rating(id, cnx)

    if package_rating is None:
        # Insert the rating into the database
        print("Inserting rating into database")
        insert_rating(id, rating, cnx)
        
        # Add time package was rated in PackageHistoryEntry
        print(f"Marking package with id = {id} as rated in PackageHistoryEntry")
        mark_as_rated(id, cnx)
    else:
        # Update the rating in the database
        print("Updating rating in database")
        update_rating(id, rating, cnx)
        
        # Add time package was updated in PackageHistoryEntry
        print("Marking id = {} as updated in PackageHistoryEntry".format(id))
        mark_as_updated(id, cnx)

    # cnx.close()

    return jsonify(rating)

# Functions to interact with the database
def mark_as_updated(id, cnx):
    query = text("INSERT INTO PackageHistoryEntry (ID, User_name, Date, Action) VALUES (:id, :username, :date, :action)")
    cnx.execute(query, parameters = {"id":id, "username":"ece30861defaultadminuser", "date":datetime.datetime.now(), "action":"UPDATE"})
    cnx.commit()

def update_rating(id, rating, cnx):
    query = text("UPDATE PackageRating SET BusFactor = :bf, Correctness = :c, RampUp = :ru, ResponsiveMaintainer = :rm, LicenseScore = :ls, GoodPinningPractice = :gpp, PullRequest = :pr, NetScore = :ns WHERE ID = :id")
    cnx.execute(query, parameters = {"id":id, "bf":rating["BusFactor"], "c":rating["Correctness"], "ru":rating["RampUp"], "rm":rating["ResponsiveMaintainer"],
                       "ls":rating["LicenseScore"], "gpp":rating["GoodPinningPractice"], "pr":rating["PullRequest"], "ns":rating["NetScore"]})
    cnx.commit()

def mark_as_rated(id, cnx):
    query = text("INSERT INTO PackageHistoryEntry (ID, User_name, Date, Action) VALUES (:id, :username, :date, :action)")
    cnx.execute(query, parameters = {"id":id, "username":"ece30861defaultadminuser", "date":datetime.datetime.now(), "action":"RATE"})
    cnx.commit()

def insert_rating(id, rating, cnx):
    query = text("INSERT INTO PackageRating (ID, BusFactor, Correctness, RampUp, ResponsiveMaintainer, LicenseScore, GoodPinningPractice, PullRequest, NetScore) VALUES (:id, :bf, :c, :ru, :rm, :ls, :gpp, :pr, :ns)")
    cnx.execute(query, parameters = {"id":id, "bf":rating["BusFactor"], "c":rating["Correctness"], "ru":rating["RampUp"], "rm":rating["ResponsiveMaintainer"],
                       "ls":rating["LicenseScore"], "gpp":rating["GoodPinningPractice"], "pr":rating["PullRequest"], "ns":rating["NetScore"]})
    cnx.commit()

def get_package_rating(id, cnx):
    query = text("SELECT * FROM PackageRating WHERE ID = :id")
    package_rating = cnx.execute(query, parameters = {"id":id}).fetchone()
    cnx.commit()
    return package_rating

def run_cli(package_url, clipath):
    rating = subprocess.check_output([clipath, package_url], env=os.environ)
    return rating

def get_package_url(id, cnx):
    query = text("SELECT URL FROM Package where ID=:id")
    package_url = cnx.execute(query, parameters = {"id":id}).fetchone()
    cnx.commit()
    return package_url