initSidebarItems({"enum":[["Algorithm","The algorithms supported for signing/verifying"]],"fn":[["decode","Decode a token into a Claims struct If the token or its signature is invalid, it will return an error"],["encode","Encode the claims passed and sign the payload using the algorithm from the header and the secret"],["sign","Take the payload of a JWT and sign it using the algorithm given. Returns the base64 url safe encoded of the hmac result"],["verify","Compares the signature given with a re-computed signature"]],"mod":[["errors",""]],"struct":[["Header","A basic JWT header part, the alg defaults to HS256 and typ is automatically set to `JWT`. All the other fields are optional"],["TokenData","The return type of a successful call to decode(...)"]],"trait":[["Part","A part of the JWT: header and claims specifically Allows converting from/to struct with base64"]]});