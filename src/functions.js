var Client_SDK = require('./Client_SDK/src/index');
let api_service = new Client_SDK.DefaultApi();


let isValid = false;
document.querySelector("#login-info").addEventListener("submit", function(e){
    let params = Array.from(document.querySelectorAll('#login-info input')).reduce((acc, input) => ({...acc,[input.id]:input.value}), {});
    if(params["username"] != '' && params["user-pw"] != '') {
        isValid = true;
    }
    if(!isValid){
        e.preventDefault();//stop form from submitting
    }
    let authModel = Client_SDK.AuthenticationRequest(params['username'], params['user-pw']);
    let authToken = api_service.createAuthToken(authModel,Client_SDK.DefaultApi().callApiCallback);
});
