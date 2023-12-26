import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static org.assertj.core.api.Assertions.*
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import internal.GlobalVariable as GlobalVariable

'STEP : Running API DELETE USERS by ID'
ResponseObject response = WS.sendRequest(findTestObject('REST-API todos/DELETE - Todos by ID', [('baseurl') : GlobalVariable.BASE_URL, ('id') : 1]))

'STEP : Verifikasi Response Code'
WS.verifyResponseStatusCode(response, 200)

'STEP : Verifikasi Value Response Code'
assertThat(response.getStatusCode()).isIn(Arrays.asList(200, 201, 202))



