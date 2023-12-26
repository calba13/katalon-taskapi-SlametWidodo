import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import internal.GlobalVariable as GlobalVariable

'STEP : Running API PATCH USERS by ID'
CustomKeywords.'finalProjectKW.apiUserFeature.patchUserbyID'(findTestObject('REST-API USERS/PATCH - User by ID', [('baseurl') : GlobalVariable.BASE_URL, ('id') : id, ('name') : name
            , ('email') : email]), id, name, email)

