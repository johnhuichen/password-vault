import React from "react";

import Link from "next/link";

export default function NavBar() {
  return (
    <div className="flex space-x-4 p-4 bg-gray-100 justify-center">
      <Link
        href="/signup"
        className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 transition-colors"
      >
        Sign up
      </Link>
      <Link
        href="/login"
        className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 transition-colors"
      >
        Log in
      </Link>
    </div>
  );
}
