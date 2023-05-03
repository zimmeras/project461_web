from flask import request, jsonify, make_response
import json
import jsonschema
from jsonschema import validate
import base64
import io
import zipfile
import os
from flask.blueprints import Blueprint
from .database import db_connect
from .auth import *
import requests
#import re


bp = Blueprint('packageid', __name__)

idvalue = 0

input_schema = {
  "type": "object",
  "properties": {
    "Content": {
      "anyOf": [
        {
          "type": "string"
        },
        {
          "type": "null"
        }
      ]
    },
    "URL": {
      "anyOf": [
        {
          "type": "string"
        },
        {
          "type": "null"
        }
      ]
    },
    "JSProgram": {
      "type": "string"
    }
  },
  "required": ["JSProgram"]
}


#TESTING
@bp.route('/package', methods=['POST'], endpoint='postEND')
#@token_required
def package():
    # Connect to database
    cnx = db_connect()
  
    global idvalue
    idvalue += 1
    # Expect JSON input
    if request.is_json:
        try:
            validate(request.json, input_schema)
            print(f"PATH (post package): {request.path} {request.method}")
            print(f"REQUEST BODY: {str(request.get_data())}")
            if request.json["Content"] == None and request.json["URL"] == None:
              print(f"post package both content and url are null")
              return make_response('', 400)
            
            if "Content" in request.json and request.json["Content"] != None:
            #if "Content" in request.json:
              content = request.json["Content"]
              jsprog = request.json["JSProgram"]
            
            
            # BELOW
              zip_file_bytes = base64.b64decode(content)
              zip_file_io = io.BytesIO(zip_file_bytes)

              with zipfile.ZipFile(zip_file_io, 'r') as zip_file:
                fileList = zip_file.namelist()
                for index, file_path in enumerate(fileList):
                  if 'package.json' in file_path:
                    package_json_bytes = zip_file.read(fileList[index])
                    package_json_str = package_json_bytes.decode('utf-8')
                    data = json.loads(package_json_str)
                    name = data['name']
                    version = data['version']
                    url = data['repository']['url']
                  else:
                    zero = 0
            #ABOVE

            if "URL" in request.json and request.json["URL"] != None:
              jsprog = request.json["JSProgram"]
              url = request.json["URL"]
              owner, repo = url.split('/')[-2:]
              response = requests.get(f"https://api.github.com/repos/{owner}/{repo}/zipball")
              content = base64.b64encode(response.content).decode('utf-8')
              package_response = requests.get(f"https://api.github.com/repos/{owner}/{repo}/contents/package.json")
              if package_response.status_code == 404:
                 print(f"no package.json was found - post package")
                 return make_response('', 424)
              # Parse the package.json file to extract the name and version of the package
              package_info = json.loads(base64.b64decode(package_response.json()['content']).decode('utf-8'))
              name = package_info['name']
              version = package_info['version']

            search_stmt = sqlalchemy.text("SELECT * FROM Package WHERE Name=:name AND Version = :version")
            package = cnx.execute(search_stmt, parameters={"name": name, "version": version}).fetchone()
            cnx.commit()

            if package:
              return make_response('', 409)

            insert_stmt = sqlalchemy.text("INSERT INTO Package (ID, Name, Version, Content, JSProgram, URL) VALUES (:idvalue, :name, :version, :content, :jsprog, :url)")
            cnx.execute(insert_stmt, parameters={"idvalue": idvalue, "name": name, "version": version, "content": content, "jsprog": jsprog, "url": url})
            cnx.commit()

            metadata = {
            "Name": name,
            "Version": version,
            "ID": idvalue
            }
            data = {
            "Content": content,
            "JSProgram": jsprog
            }
            response = {"metadata": metadata, "data": data}
            print(f"RESPONSE BODY: {response}")
            return jsonify(response), 201 
            
        except jsonschema.exceptions.ValidationError as err:
            print("schema error for post package")
            return make_response('', 400)
    else:
        return make_response('', 424)

# Function to interact with database
def post_package(content, jsprog, cur, name, version, url):
    insert_query = "INSERT INTO Package (ID, Name, Version, Content, JSProgram, URL) VALUES (%s, %s, %s, %s, %s, %s)"
    values = (idvalue, name, version, content, jsprog, url)
    cur.execute(insert_query, values)


@bp.route('/package/<int:id>', methods=['GET'], endpoint = 'getEND')
#@token_required
def get_package(id):
  print(f"PATH (get): {request.path} {request.method}")
  print(f"trying to get id = {id}")
  print(f"REQUEST BODY of get: {str(request.get_data())}")
  
  cnx = db_connect()
  
  search_stmt = sqlalchemy.text("SELECT * FROM Package WHERE ID=:id")
  package = cnx.execute(search_stmt, parameters={"id": id}).fetchone()
  cnx.commit()

  if package is None:
    print(f"package wanted to get doesn't exist")
    return make_response('', 404)
  
  metadata = {
    "Name": package[1],
    "Version": package[2],
    "ID": package[0]
  }
  data = {
    "Content": package[3],
    "JSProgram": package[5]
  }
  response = {"metadata": metadata, "data": data}
  print(f"RESPONSE (get): {response}")
  result = make_response(response, 200)
  return result

# Function to interact with database
def get_package(id, cnx):

    search_stmt = sqlalchemy.text("SELECT * FROM Package WHERE ID=:id")
    package = cnx.execute(search_stmt, parameters={"id": id}).fetchone()
    cnx.commit()
    return package

input_schema2 = {
  "type": "object",
  "properties": {
    "metadata": {
      "type": "object",
      "properties": {
        "Name": {
          "type": "string"
        },
        "Version": {
          "type": "string"
        },
        "ID": {
          "type": "string"
        }
      },
      "required": [
        "Name",
        "Version",
        "ID"
      ]
    },
    "data": {
      "type": "object",
      "properties": {
        "Content": {
          "anyOf": [
            {"type": "string"},
            {"type": "null"}
          ]
        },
        "URL": {
          "anyOf": [
            {"type": "string"},
            {"type": "null"}
          ]
        },
        "JSProgram": {
          "type": "string"
        }
      },
      "required": [
        "JSProgram"
      ]
    }
  },
  "required": [
    "metadata",
    "data"
  ]
}

@bp.route('/package/<int:id>', methods=['PUT'], endpoint = 'putEND')
#@token_required
def put_package(id):
  # Connect to database
  print(f"PATH (put package): {request.path} {request.method}")
  print(f"REQUEST BODY put: {str(request.get_data())}")
  if(len(str(request.get_data())) == 0):
     print(f"no request body for update package id = {id}")
  cnx = db_connect()

  if request.is_json:
    try:
      jsonschema.validate(instance=request.json, schema=input_schema2)
  
      #check if package exists 
      package = get_package(id, cnx)

      if package is None:
        print(f"searched package doesn't exist {id}")
        return make_response('', 404)
      
      #get input data 
      metadata = request.json["metadata"]
      name = metadata["Name"]
      version = metadata["Version"]
      metadata = request.json["data"]
      content = metadata["Content"]
      url = metadata["URL"]
      jsprogram = metadata["JSProgram"]

      if url == None and content == None:
        print(f"both url and content is null for update package {id}")
        return make_response('', 400)

      if url == None:
        zip_file_bytes = base64.b64decode(content)
        zip_file_io = io.BytesIO(zip_file_bytes)

        with zipfile.ZipFile(zip_file_io, 'r') as zip_file:
          fileList = zip_file.namelist()
          for index, file_path in enumerate(fileList):
            if 'package.json' in file_path:
              package_json_bytes = zip_file.read(fileList[index])
              package_json_str = package_json_bytes.decode('utf-8')
              data = json.loads(package_json_str)
              url = data['homepage']
            else:
              zero = 0

      if content == None:
        owner, repo = url.split('/')[-2:]
        response = requests.get(f"https://api.github.com/repos/{owner}/{repo}/zipball")
        content = base64.b64encode(response.content).decode('utf-8')
      #update
      update_package(id, cnx, name, version, content, url, jsprogram)

      return make_response('', 200)
    except jsonschema.exceptions.ValidationError as err:
      print("update package schema doesnt match {id}")
      return make_response('', 400)
  else:
        return make_response('', 424)

def update_package(id, cnx, name, version, content, url, jsprogram):

    search_stmt = sqlalchemy.text("UPDATE Package SET Name =:name, Version =:version, Content =:content, URL =:url, JSProgram =:jsprogram WHERE ID =:id")
    cnx.execute(search_stmt, parameters={"name": name, "version": version, "content": content, "url": url, "jsprogram": jsprogram, "id": id})
    cnx.commit()
   

@bp.route('/package/<int:id>', methods=['DELETE'], endpoint = 'deleteEND')
#@token_required
def delete_package(id):
  print(f"PATH (delete id): {request.path} {request.method}")
  print(f"REQUEST BODY: {str(request.get_data())}")
  # Connect to database
  cnx = db_connect()

  # Get package from the database
  package = get_package(id, cnx)

  if package is None:
    return make_response('', 404)
  
  delete_from_db(id, cnx)
  return make_response('', 200)

# Function to interact with database
def delete_from_db(id, cnx):
    search_stmt = sqlalchemy.text("DELETE FROM Package WHERE ID =:id")
    cnx.execute(search_stmt, parameters={"id": id})
    cnx.commit()
    '''
    cnx.reconnect()
    cur = cnx.cursor()
    cur.execute("DELETE FROM Package WHERE ID = %s", (id,))
    cnx.commit()
    cur.close()
    cnx.close()
    '''


@bp.route('/reset', methods=['DELETE'], endpoint = 'resetEND')
#@token_required
def reset_package():
  print(f"PATH (/reset): {request.path} {request.method}")
  print(f"REQUEST BODY: {str(request.get_data())}")
  cnx = db_connect()

  search_stmt = sqlalchemy.text("DELETE FROM Package")
  cnx.execute(search_stmt)
  cnx.commit()
  '''
  cnx.reconnect()
  cur = cnx.cursor()
  cur.execute("DELETE FROM Package")
  cnx.commit()
  cur.close()
  cnx.close()
  '''
  return make_response('', 200)

input_schema4 = {
  "type": "object",
  "properties": {
    "RegEx": {
      "type": "string",
    }
  },
  "required": ["RegEx"]
}


@bp.route('/package/byRegEx', methods=['POST'], endpoint = 'regExEND')
#@token_required
def regex_package():
  return make_response('', 502)
'''
  print(f"PATH (POST RegEx): {request.path} {request.method}")
  print(f"REQUEST BODY: {str(request.get_data())}")
  cnx = db_connect()

  if request.is_json:
    try:
      validate(request.json, input_schema4)
      #get input data 
      reg = request.json["RegEx"]
      search_stmt = sqlalchemy.text("SELECT * FROM Package WHERE name REGEXP :reg")
      results = cnx.execute(search_stmt, parameters={":reg": reg}).fetchall()
      cnx.commit()

      packages = []
      for row in results:
        package = {'Version': row[0], 'Name': row[1]}
        packages.append(package)
      print(f"RESPONSE BODY: {packages}")
      return make_response(packages, 200)
    except jsonschema.exceptions.ValidationError as err:
      return make_response('', 400)
  else:
        return make_response('', 404)
'''

@bp.route('/packages', methods=['POST'], endpoint = 'packagesExEND')
#@token_required
def regex_package():
  print(f"PATH (/packages): {request.path} {request.method}")
  print(f"REQUEST BODY (packages): {str(request.get_data())}")
  return make_response('',200)