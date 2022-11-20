<?php
function isPrime($num) {
  if ($num <= 1) {
    return false;
  }
  for ($i = 2; $i <= $num / 2; $i++) {
    if ($num % $i == 0) {
      return false;
    }
  }
  return true;
}

function findPrime($num) {
  $largestPrime = 0;
  for ($j = 2; $j <= $num; $j++) {
    if (isPrime($j)) {
      $largestPrime = $j;
    }
  }

  return $largestPrime;
}


$now = time();
$prime = findPrime(500000);
echo("prime = ".$prime);
echo("\r\n");
echo("elapsed: ".(time() - $now));
?>