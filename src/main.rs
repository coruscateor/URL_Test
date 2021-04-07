use url::{Url, ParseError};

/*

The Url llbrary appends a forward slash to the end of the provided URL if none is present

http://example.com/
http://example.com/
http://something.com/
http://something.com/
http://test.something.com/
http://test.something.com/

Done

*/

fn main() {

    checkAbdPrint(Url::parse("http://example.com"));

    checkAbdPrint(Url::parse("http://example.com/"));

    checkAbdPrint(Url::parse("http://something.com"));

    checkAbdPrint(Url::parse("http://something.com/"));

    checkAbdPrint(Url::parse("http://test.something.com"));

    checkAbdPrint(Url::parse("http://test.something.com/"));

    //checkAbdPrint(Url::parse("1234"));

    println!("\nDone\n");

}

fn checkAbdPrint(result: Result<Url, ParseError>)
{

    match result
    {

        Ok(res) => {

            println!("{}", res.as_str());

        },
        Err(err) => {

            parseErrorOut(err);

        }

    }

}

fn parseErrorOut(err:  ParseError)
{

    match err
    {

        ParseError::EmptyHost => {

            println!("EmptyHost");

        },
        ParseError::IdnaError => {

            println!("IdnaError");

        },
        ParseError::InvalidPort => {

            println!("InvalidPort");

        },
        ParseError::InvalidIpv4Address => {

            println!("InvalidIpv4Address");            

        },
        ParseError::InvalidIpv6Address => {

            println!("InvalidIpv6Address");

        },
        ParseError::InvalidDomainCharacter => {

            println!("InvalidDomainCharacter");

        },
        ParseError::RelativeUrlWithoutBase => {

            println!("RelativeUrlWithoutBase");

        },
        ParseError::RelativeUrlWithCannotBeABaseBase => {

            println!("RelativeUrlWithCannotBeABaseBase");

        },
        ParseError::SetHostOnCannotBeABaseUrl => {

            println!("SetHostOnCannotBeABaseUrl");

        },
        ParseError::Overflow => {

            println!("Overflow");

        },
        ParseError::__FutureProof => {

            println!("__FutureProof");

        }
    }

}
