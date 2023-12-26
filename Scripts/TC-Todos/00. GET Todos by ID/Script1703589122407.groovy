import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import internal.GlobalVariable as GlobalVariable

'STEP : Running API GET TODOs by ID'
CustomKeywords.'finalProjectKW.apiTodosFeature.getTodosbyID'(findTestObject('REST-API todos/GET - Todos by ID', [('baseurl') : GlobalVariable.BASE_URL, ('id') : id]),
	id, title, completed)