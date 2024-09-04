
import LoginSignup from "@/components/loginSignup";

export default function LoginSignupPage({ params }: { params: { loginSignup: string } }){
    return (
        <LoginSignup params={params.loginSignup} />
    )
}
export const generateStaticParams: GetStaticParams = async () => {
    return [
      { loginSignup: 'login' },
      { loginSignup: 'signup' },
    ];
  };
