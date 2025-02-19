from webapp import client
# from __init__ import login
import mock
import pytest
import json

def post_schema(content, JSprogram):
    schema = {"Content": content, "JSProgram": JSprogram}
    return schema

def put_schema():
    schema = {"metadata": {"Name": "string","Version": "1.2.3","ID": "string"},"data": {"Content": "string","URL": "string","JSProgram": "string"}}
    return schema

# Mock all database interactions in packageid.py
mock_get_package = (101, 'Cloudinary', None, None, 'https://github.com/cloudinary/cloudinary_npm', None)


@mock.patch('src.APIs.packageid.get_package', return_value=mock_get_package)
@mock.patch('src.APIs.packageid.post_package') 
@pytest.mark.parametrize("schema, status_code", [
    (post_schema("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", 
                 "if (process.argv.length === 7) {\nconsole.log('Success')\nprocess.exit(0)\n} else {\nconsole.log('Failed')\nprocess.exit(1)\n}\n"), 200)
])
def test_mock_post_packageid(mock_post, mock_get, client, schema, status_code):
    #add Content-Type: application/json to header
    # add schema to body 
    landing = client.post("/package", headers={'Content-Type': 'application/json'}, json=schema)
    html = landing.data.decode()
    assert landing.status_code == status_code

@mock.patch('src.APIs.packageid.get_package', return_value=mock_get_package)
@mock.patch('src.APIs.packageid.update_package')
@pytest.mark.parametrize("schema, status_code", [
    (put_schema(), 200),
])
def test_mock_put_packageid(mock_update, mock_get, client, schema, status_code):
    landing = client.put("/package/101", headers={'Content-Type': 'application/json'}, json=schema)
    assert landing.status_code == status_code

@mock.patch('src.APIs.packageid.get_package', return_value=mock_get_package)
@pytest.mark.parametrize("status_code", [(200)])
def test_mock_get_packageid(mock_get, client, status_code):
    landing = client.get("/package/101")
    assert landing.status_code == status_code

@mock.patch('src.APIs.packageid.get_package', return_value=mock_get_package)
@mock.patch('src.APIs.packageid.delete_from_db')
@pytest.mark.parametrize("status_code", [(200)])
def test_mock_delete_packageid(mock_delete, mock_get, client, status_code):
    landing = client.delete("/package/101")
    assert landing.status_code == status_code