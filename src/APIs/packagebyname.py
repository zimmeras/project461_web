'''
This file contains the following api calls:
    - /package/byname/<string:Name> (GET)
    - /package/byname/<string:Name> (DELETE)
'''
from .auth import *
from flask import abort
from flask.blueprints import Blueprint
from .database import db_connect
from .auth import *
from sqlalchemy.sql import text
bp = Blueprint('packagebyname', __name__)

# Returns all packages with name = Name from the database
@bp.route('/package/byname/<string:Name>', methods=['GET'], endpoint='rateGET')
#@token_required
def get_package_by_name(Name):
    cursor = db_connect()

    # Get all ids with name = Name
    ids = get_ids(Name, cursor)

    # If the package doesn't exist, return a 404
    if len(ids) == 0:
        print("package does not exist, returning 404")
        abort(404)

    # For each id, get the package entry history from PackageEntryHistory
    package_entry_history = []
    packageshistory = []
    for id in ids:
        id = id[0]
        package_entry_history.append(get_package_entry(cursor, id))
        # print(package_entry_history)
        # Get user data for each package entry
        for i in range(len(package_entry_history[-1])):
            userdata = get_user_data(cursor, package_entry_history, i)
            version = get_version(cursor, package_entry_history, i)
            # Combine the user data with the package entry history in correct format
            user = {"name": userdata[0][0], "isAdmin": userdata[0][1]}
            metadata = {"Name": Name, "Version": version,
                       "ID": package_entry_history[-1][i][0]}

            packagehistory = {"User": user, "Date": package_entry_history[-1][i][2],
                              "PackageMetadata": metadata, "Action": package_entry_history[-1][i][3]}
            packageshistory.append(packagehistory)

    return ((packageshistory))

# Functions used to interact with the database
def get_version(cursor, package_entry_history, i):
    query = text("SELECT Version FROM Package WHERE ID =:entry")
    version = cursor.execute(query, {"entry":package_entry_history[-1][i][0]}).fetchone()
    return version

def get_user_data(cursor, package_entry_history, i):
    query = text("SELECT name, isAdmin FROM User WHERE name =:entry")
    userdata = cursor.execute(query, parameters = {"entry":package_entry_history[-1][i][1]}).fetchall()
    return userdata

def get_package_entry(cursor, id):
    query = text("SELECT * FROM PackageHistoryEntry WHERE ID =:id")
    entry = cursor.execute(query, parameters={"id":id}).fetchall()
    return entry

def get_ids(name, cursor):
    query = text("SELECT ID FROM Package WHERE name =:Name")
    ids = cursor.execute(query, parameters={"Name":name}).fetchall()
    return ids

# Deletes all instances of a packages with name = Name from the database
# Deletes entries in PackageEntryHistory, Package, PackageRating
@bp.route('/package/byname/<string:Name>', methods=['DELETE'], endpoint='rateDELETE')
#@token_required
def delete_package_by_name(Name):
    cursor = db_connect()

    # Get all ids with name = Name
    ids = get_ids(Name, cursor)

    # If the package doesn't exist, return a 404
    if len(ids) == 0:
        print("package does not exist, returning 404")
        cursor.close()
        abort(404)

    # Delete all instances where id = id from PackageRating
    for id in ids:
        delete_from_PackageRating(cursor, id)

        # Delete all instances where id = id from PackageEntryHistory
        print("Deleting id = {} from PackageHistoryEntry".format(id))
        delete_from_PackageEntryHistory(cursor, id)

    # Delete all instances where name = Name from Package
    print("delete from Package where Name = %s", Name)
    delete_from_Package(Name, cursor)
    cursor.close()
    return '', 200

# Functions used to interact with the database
def delete_from_Package(Name, cursor):
    query = text("DELETE FROM Package WHERE Name =:Name")
    cursor.execute(query, parameters = {"Name":Name})

def delete_from_PackageEntryHistory(cursor, id):
    query = text("DELETE FROM PackageHistoryEntry WHERE ID =:id")
    cursor.execute(query, parameters = {"id":id})

def delete_from_PackageRating(cursor, id):
    print("Deleting id = {} from PackageRating".format(id))
    query = text("DELETE FROM PackageRating WHERE ID =:id")
    cursor.execute(query, parameters = {"id":id})