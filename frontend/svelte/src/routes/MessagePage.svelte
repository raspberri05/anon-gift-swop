<script context="module">
  export function load({ params }) {
    return {
      props: {
        message: params.msg || ''
      }
    };
  }

</script>
<script>

  export let message;

  function decodeMessage(encodedMessage) {
    try {
      // Decode the Base64 string and return
      return atob(encodedMessage);
    } catch (error) {
      console.error("Failed to decode message:", error);
      return "Invalid message format";
    }
  }

  const decodedMessage = decodeMessage(message);

</script>

<style>
  p {
    font-size: 1.2em;        /* Slightly larger font size for the message */
    color: #555;             /* Softer color for the message */
    line-height: 1.5;        /* Improved line height for readability */
    background-color: #f9f9f9; /* Light background for contrast */
    border: 1px solid #ddd;   /* Subtle border */
    border-radius: 5px;      /* Rounded corners */
    padding: 15px;           /* Padding for space inside */
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1); /* Light shadow for depth */
  }
</style>

<p>{decodedMessage}</p>
