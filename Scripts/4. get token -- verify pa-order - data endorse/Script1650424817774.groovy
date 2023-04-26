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
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil

token = WS.sendRequest(findTestObject('get-token'))

'store json response to variable'
def slurper = new groovy.json.JsonSlurper()

def get_token = slurper.parseText(token.getResponseBodyContent())

'define / field api'
def private_token = get_token.access_token

'store var to global variable'
GlobalVariable.global_token = private_token

println('token is :' + GlobalVariable.global_token)

'show response to report in test suite'
response_token = token.getResponseText()

KeywordUtil.logInfo("$response_token")

order = WS.sendRequest(findTestObject('pa-order', [('token') : private_token, ('order_date') : order_date, ('maskapai_id') : maskapai_id
            , ('page') : page, ('row') : row, ('source') : source]))

'show response to report in test suite'
get_order = order.getResponseText()


KeywordUtil.markPassed ('verifikasi data ke : ' + no)

'verifikasi data konsumen'
WS.verifyElementPropertyValue(order, 'Data.BelumTerimaPolis[0].Konsumen.OrderID', k_orderid, FailureHandling.CONTINUE_ON_FAILURE)

WS.verifyElementPropertyValue(order, 'Data.BelumTerimaPolis[0].Konsumen.AppID', k_appid, FailureHandling.CONTINUE_ON_FAILURE)

WS.verifyElementPropertyValue(order, 'Data.BelumTerimaPolis[0].Konsumen.NoKontrak', k_nokontrak, FailureHandling.CONTINUE_ON_FAILURE)

WS.verifyElementPropertyValue(order, 'Data.BelumTerimaPolis[0].Konsumen.Nama', k_nama, FailureHandling.CONTINUE_ON_FAILURE)

WS.verifyElementPropertyValue(order, 'Data.BelumTerimaPolis[0].Konsumen.JenisKelamin', k_jeniskelamin, FailureHandling.CONTINUE_ON_FAILURE)

WS.verifyElementPropertyValue(order, 'Data.BelumTerimaPolis[0].Konsumen.NIK', k_nik, FailureHandling.CONTINUE_ON_FAILURE)

WS.verifyElementPropertyValue(order, 'Data.BelumTerimaPolis[0].Konsumen.TempatLahir', k_tempatlahir, FailureHandling.CONTINUE_ON_FAILURE)

WS.verifyElementPropertyValue(order, 'Data.BelumTerimaPolis[0].Konsumen.TanggalLahir', k_tgllahir, FailureHandling.CONTINUE_ON_FAILURE)

WS.verifyElementPropertyValue(order, 'Data.BelumTerimaPolis[0].Konsumen.Alamat.DetailAlamat', k_alamat, FailureHandling.CONTINUE_ON_FAILURE)

WS.verifyElementPropertyValue(order, 'Data.BelumTerimaPolis[0].Konsumen.Alamat.Kelurahan', k_kel, FailureHandling.CONTINUE_ON_FAILURE)

WS.verifyElementPropertyValue(order, 'Data.BelumTerimaPolis[0].Konsumen.Alamat.Kecamatan', k_kec, FailureHandling.CONTINUE_ON_FAILURE)

WS.verifyElementPropertyValue(order, 'Data.BelumTerimaPolis[0].Konsumen.Alamat.KabupatenKota', k_kab, FailureHandling.CONTINUE_ON_FAILURE)

WS.verifyElementPropertyValue(order, 'Data.BelumTerimaPolis[0].Konsumen.Alamat.Provinsi', k_prov, FailureHandling.CONTINUE_ON_FAILURE)

WS.verifyElementPropertyValue(order, 'Data.BelumTerimaPolis[0].Konsumen.Alamat.KodePos', k_zipcode, FailureHandling.CONTINUE_ON_FAILURE)

WS.verifyElementPropertyValue(order, 'Data.BelumTerimaPolis[0].Konsumen.Perkerjaan', k_pekerjaan, FailureHandling.CONTINUE_ON_FAILURE)

WS.verifyElementPropertyValue(order, 'Data.BelumTerimaPolis[0].Konsumen.TempatKonsumenBekerja', k_tempatkonsumenbekerja, FailureHandling.CONTINUE_ON_FAILURE)

WS.verifyElementPropertyValue(order, 'Data.BelumTerimaPolis[0].Polis.TanggalMulaiPolis', k_TanggalMulaiPolis, FailureHandling.CONTINUE_ON_FAILURE)

WS.verifyElementPropertyValue(order, 'Data.BelumTerimaPolis[0].Polis.TanggalSelesaiPolis', k_TanggalSelesaiPolis, FailureHandling.CONTINUE_ON_FAILURE)

WS.verifyElementPropertyValue(order, 'Data.BelumTerimaPolis[0].Polis.Rate', k_Rate, FailureHandling.CONTINUE_ON_FAILURE)

WS.verifyElementPropertyValue(order, 'Data.BelumTerimaPolis[0].Polis.TotalSumInsured', k_TotalSumInsured, FailureHandling.CONTINUE_ON_FAILURE)