package internal

import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.main.TestCaseMain


/**
 * This class is generated automatically by Katalon Studio and should not be modified or deleted.
 */
public class GlobalVariable {
     
    /**
     * <p>Profile Dev_env : The Base REST URL
Profile Prod_env : The Base REST URL</p>
     */
    public static Object URL
     
    /**
     * <p></p>
     */
    public static Object IssueKey
     
    /**
     * <p></p>
     */
    public static Object bugParams
     

    static {
        try {
            def selectedVariables = TestCaseMain.getGlobalVariables("default")
			selectedVariables += TestCaseMain.getGlobalVariables(RunConfiguration.getExecutionProfile())
            selectedVariables += RunConfiguration.getOverridingParameters()
    
            URL = selectedVariables['URL']
            IssueKey = selectedVariables['IssueKey']
            bugParams = selectedVariables['bugParams']
            
        } catch (Exception e) {
            TestCaseMain.logGlobalVariableError(e)
        }
    }
}
