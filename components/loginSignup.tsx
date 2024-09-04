"use client";
import { useState, ChangeEvent, FormEvent } from "react";
import { useRouter } from 'next/navigation';
import {invoke} from '@tauri-apps/api/tauri'
interface LoginSignupProps {
    params: string
  }

export default function LoginSignup({ params }: LoginSignupProps){
    const router = useRouter()
    const [masterKey, setMasterKey] = useState('');
    const [errors, setErrors] = useState([])
    const handleMasterKey = (e: ChangeEvent<HTMLInputElement>) => {
        setMasterKey(e.target.value);
      };
    const handleClick = ()=>{
        router.push('/')
    }
    const handleSubmit = (e: FormEvent<HTMLFormElement>)=>{
        e.preventDefault()
        if(params === 'signup'){
            console.log('signup', masterKey)
        }else{
            console.log('login', masterKey)
        }
    };
    return (
        <>
            <div className='text-center my-4'>
                {params}
            </div>
            <div className='p-10 m-10 border-2 border-solid border-black rounded-md bg-gray-50 max-w-md mx-auto'>
                <form onSubmit={handleSubmit} className='space-y-4'>
            <div>
            <label className='block text-lg font-semibold mb-2'>Masterkey:</label>
            <input
                type='password'
                onChange={handleMasterKey}
                value={masterKey}
                className='w-full px-4 py-2 border border-gray-300 rounded focus:outline-none focus:border-blue-500'
            />
            </div>
            <input
                type='submit'
                value='Submit'
                className='bg-blue-500 text-white px-4 py-2 rounded cursor-pointer hover:bg-blue-600 transition-colors w-full'
            />
                </form>
            </div>
            <button
                value='back'
                onClick={handleClick}
                className='mt-4 bg-gray-500 text-white px-4 py-2 rounded hover:bg-gray-600 transition-colors'
            >
                Back
            </button>

        </>
    )
}
