import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject
import com.kms.katalon.core.util.KeywordUtil
import com.kms.katalon.core.testobject.HttpMessage
import com.kms.katalon.core.testobject.RequestObject as RequestObject
import com.kms.katalon.core.webservice.verification.WSResponseManager as WSResponseManager
import com.kms.katalon.core.testobject.impl.HttpTextBodyContent as HttpTextBodyContent

token = WS.sendRequest(findTestObject('get-token'))

'store json response to variable'
def slurper = new groovy.json.JsonSlurper()

def get_token = slurper.parseText(token.getResponseBodyContent())

'define / field api'
def private_token = get_token.access_token

'store var to global variable'
GlobalVariable.global_token = private_token

'show response to report in test suite'
response_token = token.getResponseText()

KeywordUtil.logInfo("$response_token")

println('token is :' + GlobalVariable.global_token)

def feedback = (RequestObject)findTestObject('pa-feedback', [('token') : private_token, ('maskapai_id') : maskapai_id
            , ('order_id') : order_id, ('no_polis') : no_polis, ('base64') : base64, ('source') : source])


'menambahkan body request'

//call request body content
HttpTextBodyContent body2 = feedback.getBodyContent()

//Variable get request body text
String req = body2.getText()

'Cek response data'

ResponseObject response = WS.sendRequest(feedback)

'show response to report in test suite'
String response_feedback = response.getResponseText()

'-------------------------------------------------------'

KeywordUtil.markPassed ('Hit data ke : ' + no)

'print log request text body'
KeywordUtil.logInfo("$req")

'print log response text body'
KeywordUtil.logInfo("$response_feedback")

